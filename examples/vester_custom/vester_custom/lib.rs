// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Contract Summary:
/// A custom vester contract that uses GeneralVest trait.
// ########################################################
// inject GeneralVest trait's default implementation (GeneralVestDefaultImpl & GeneralVestInternalDefaultImpl)
#[pendzl::implementation(GeneralVest)]
#[ink::contract]
pub mod vester_custom {
    #[ink(storage)]
    // derive explained below
    #[derive(Default, StorageFieldGetter)]
    pub struct Vester {
        // apply the storage_field attribute so it's accessible via `self.data::<GeneralVest>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // GeneralVestData is a struct that implements GeneralVestStorage - required by GeneralVestInternalDefaultImpl trait
        // note it's not strictly required by GeneralVest trait - just the default implementation
        // name of the field is arbitrary
        general_vest: GeneralVestData,
    }

    impl Vester {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
pub mod tests {
    use crate::vester_custom::{VesterRef, VestingSchedule, *};
    use ink::{env::DefaultEnvironment, scale::Decode as _, ToAccountId};
    use ink_e2e::{events::ContractEmitted, ChainBackend, ContractsBackend};
    use my_psp22_mintable::my_psp22_mintable::{ContractRef as PSP22Ref, *};
    use pendzl::{
        contracts::token::psp22::{Transfer, PSP22},
        traits::{AccountId, Balance},
    };
    use test_helpers::{assert_eq_msg, keypair_to_account};
    use ts_provider::ts_provider::*;

    pub const ONE_HOUR: u64 = 60 * 60 * 1000;
    pub const ONE_DAY: u64 = 24 * ONE_HOUR;

    ///////////////////////////
    struct CreateVestingScheduleArgs {
        vest_to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        schedule: VestingSchedule,
    }

    fn assert_vesting_scheduled_event(
        event: &ContractEmitted<DefaultEnvironment>,
        expected_creator: AccountId,
        expected_receiver: AccountId,
        expected_asset: Option<AccountId>,
        expected_amount: Balance,
        expected_schedule: VestingSchedule,
    ) {
        let VestingScheduled {
            creator,
            asset,
            receiver,
            amount,
            schedule,
        } = <VestingScheduled>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        assert_eq_msg!("Asset", asset, expected_asset);
        assert_eq_msg!("creator", creator, expected_creator);
        assert_eq_msg!("receiver", receiver, expected_receiver);
        assert_eq_msg!("Amounts", amount, expected_amount);
        assert_eq_msg!("schedule", schedule, expected_schedule);
    }

    fn assert_psp22_transfer_event<
        E: ink::env::Environment<AccountId = AccountId>,
    >(
        event: &ContractEmitted<E>,
        expected_from: AccountId,
        expected_to: AccountId,
        expected_value: Balance,
        expected_asset: AccountId,
    ) {
        let decoded_event = <Transfer>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        let Transfer { from, to, value } = decoded_event;

        assert_eq_msg!("Transfer.from", from, Some(expected_from));
        assert_eq_msg!("Transfer.to", to, Some(expected_to));
        assert_eq_msg!("Transfer.value", value, expected_value);
        assert_eq_msg!("Transfer.asset", event.contract, expected_asset);
    }

    fn assert_token_released_event_e2e<
        E: ink::env::Environment<AccountId = AccountId>,
    >(
        event: &ContractEmitted<E>,
        expected_caller: AccountId,
        expected_to: AccountId,
        expected_asset: Option<AccountId>,
        expected_amount: Balance,
    ) {
        let TokenReleased {
            caller,
            asset,
            to,
            amount,
        } = <TokenReleased>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        assert_eq_msg!("caller", caller, expected_caller);
        assert_eq_msg!("Assets", asset, expected_asset);
        assert_eq_msg!("To", to, expected_to);
        assert_eq_msg!("Amounts", amount, expected_amount);
    }
    ///////////////////////////

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn release_psp22_incorrect_account_id(
        mut client: ink_e2e::Client<C, E>,
    ) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;
        let psp22_mintable_creator = ink_e2e::bob();
        let mut psp22_constructor = PSP22Ref::new(1_000_000);
        let mut psp22 = client
            .instantiate(
                "my_psp22_mintable",
                &psp22_mintable_creator,
                &mut psp22_constructor,
            )
            .submit()
            .await
            .expect("instantiate psp22 failed")
            .call_builder::<Contract>();

        let vest_to = ink_e2e::charlie();

        let amount = 5000_u128;

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: Some(psp22.to_account_id()),
            amount,
            schedule: VestingSchedule::External(ExternalTimeConstraint {
                account: keypair_to_account(&ink_e2e::dave()),
                fallback_values: (123, 456),
            }),
        };

        let mut vester_constructor = VesterRef::new();
        let mut vester = client
            .instantiate(
                "vester_custom",
                &vester_creator,
                &mut vester_constructor,
            )
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        let _ = client
            .call(
                &vester_submitter,
                &psp22.increase_allowance(
                    vester.to_account_id(),
                    create_vest_args.amount,
                ),
            )
            .submit()
            .await
            .expect("give allowance failed")
            .return_value();

        let _ = client
            .call(
                &vester_creator,
                &psp22.mint(
                    keypair_to_account(&vester_submitter),
                    create_vest_args.amount,
                ),
            )
            .submit()
            .await
            .expect("mint failed");

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .submit()
            .await
            .expect("create vest failed");

        let contract_emitted_events =
            create_vest_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == vester.to_account_id()
            })
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22.to_account_id()
            })
            .collect();
        assert!(matches!(create_vest_res.return_value(), Ok(())));

        assert_psp22_transfer_event(
            &psp22_events[1].event, //psp22 transfer emits 2 events, here we check for the actual Transfer event
            keypair_to_account(&vester_submitter),
            vester.to_account_id(),
            create_vest_args.amount,
            psp22.to_account_id(),
        );
        assert_vesting_scheduled_event(
            &vester_events[0].event,
            keypair_to_account(&vester_submitter),
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            create_vest_args.amount,
            VestingSchedule::External(ExternalTimeConstraint {
                account: keypair_to_account(&ink_e2e::dave()),
                fallback_values: (123, 456),
            }),
        );

        let release_res = client
            .call(
                &vest_to,
                &vester.release(
                    Some(create_vest_args.vest_to),
                    create_vest_args.asset,
                    vec![],
                ),
            )
            .dry_run()
            .await?
            .return_value();

        assert_eq!(release_res, Ok(()),);

        Ok(())
    }

    #[ink_e2e::test]
    async fn shorten_durations(
        mut client: ink_e2e::Client<C, E>,
    ) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;
        let psp22_mintable_creator = ink_e2e::bob();
        let mut psp22_constructor = PSP22Ref::new(1_000_000);
        let mut psp22 = client
            .instantiate(
                "my_psp22_mintable",
                &psp22_mintable_creator,
                &mut psp22_constructor,
            )
            .submit()
            .await
            .expect("instantiate psp22 failed")
            .call_builder::<Contract>();

        let vest_to = ink_e2e::charlie();

        let mut ts_provider_constructor = TSProviderRef::new(0, 0);
        let mut ts_provider = client
            .instantiate(
                "ts_provider",
                &vester_creator,
                &mut ts_provider_constructor,
            )
            .submit()
            .await
            .expect("instantiate ts_provider failed")
            .call::<TSProvider>();

        let current_ts = client
            .call(&ink_e2e::alice(), &ts_provider.get_current_timestamp())
            .dry_run()
            .await?
            .return_value();

        let start_time =
            current_ts.checked_add(ONE_DAY).ok_or(MathError::Overflow)?;
        let end_time = current_ts
            .checked_add(ONE_DAY * 10)
            .ok_or(MathError::Overflow)?;
        let amount = end_time - start_time;

        let _ = client
            .call(
                &vester_submitter,
                &ts_provider.set_waiting_duration(start_time),
            )
            .submit()
            .await
            .expect("set_waiting_duration failed")
            .return_value();
        let _ = client
            .call(
                &vester_submitter,
                &ts_provider.set_vesting_duration(end_time),
            )
            .submit()
            .await
            .expect("set_vesting_duration failed")
            .return_value();

        let schedule = VestingSchedule::External(ExternalTimeConstraint {
            account: ts_provider.to_account_id(),
            fallback_values: (start_time, end_time),
        });

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: Some(psp22.to_account_id()),
            amount: amount.into(),
            schedule,
        };

        let mut vester_constructor = VesterRef::new();
        let mut vester = client
            .instantiate(
                "vester_custom",
                &vester_creator,
                &mut vester_constructor,
            )
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        let _ = client
            .call(
                &vester_submitter,
                &psp22.increase_allowance(
                    vester.to_account_id(),
                    create_vest_args.amount,
                ),
            )
            .submit()
            .await
            .expect("give allowance failed")
            .return_value();

        let _ = client
            .call(
                &vester_creator,
                &psp22.mint(
                    keypair_to_account(&vester_submitter),
                    create_vest_args.amount,
                ),
            )
            .submit()
            .await
            .expect("mint failed");

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .submit()
            .await
            .expect("create vest failed");

        let contract_emitted_events =
            create_vest_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == vester.to_account_id()
            })
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22.to_account_id()
            })
            .collect();
        assert!(matches!(create_vest_res.return_value(), Ok(())));

        assert_psp22_transfer_event(
            &psp22_events[1].event, //psp22 transfer emits 2 events, here we check for the actual Transfer event
            keypair_to_account(&vester_submitter),
            vester.to_account_id(),
            create_vest_args.amount,
            psp22.to_account_id(),
        );
        assert_vesting_scheduled_event(
            &vester_events[0].event,
            keypair_to_account(&vester_submitter),
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            create_vest_args.amount,
            create_vest_args.schedule.clone(),
        );

        let release_res = client
            .call(
                &vest_to,
                &vester.release(
                    Some(create_vest_args.vest_to),
                    create_vest_args.asset,
                    vec![],
                ),
            )
            .submit()
            .await
            .expect("release failed");

        let contract_emitted_events = release_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == vester.to_account_id()
            })
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22.to_account_id()
            })
            .collect();

        let return_value = release_res.return_value();
        assert!(
            return_value.is_ok(),
            "release failed. res: {:?}",
            return_value
        );

        assert_psp22_transfer_event(
            &psp22_events[0].event,
            vester.to_account_id(),
            keypair_to_account(&vest_to),
            0,
            psp22.to_account_id(),
        );
        assert_token_released_event_e2e(
            &vester_events[0].event,
            create_vest_args.vest_to,
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            0,
        );

        // modify start/end times so release function releases whole general_vest amount
        let _ = client
            .call(&vester_submitter, &ts_provider.set_waiting_duration(1))
            .submit()
            .await
            .expect("set_waiting_duration failed")
            .return_value();
        let _ = client
            .call(&vester_submitter, &ts_provider.set_vesting_duration(2))
            .submit()
            .await
            .expect("set_vesting_duration failed")
            .return_value();

        let release_res = client
            .call(
                &vest_to,
                &vester.release(
                    Some(create_vest_args.vest_to),
                    create_vest_args.asset,
                    vec![],
                ),
            )
            .submit()
            .await
            .expect("release failed");

        let contract_emitted_events = release_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == vester.to_account_id()
            })
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22.to_account_id()
            })
            .collect();

        let return_value = release_res.return_value();
        assert!(
            return_value.is_ok(),
            "release failed. res: {:?}",
            return_value
        );

        assert_psp22_transfer_event(
            &psp22_events[0].event,
            vester.to_account_id(),
            keypair_to_account(&vest_to),
            amount.into(),
            psp22.to_account_id(),
        );
        assert_token_released_event_e2e(
            &vester_events[0].event,
            create_vest_args.vest_to,
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            amount.into(),
        );

        Ok(())
    }
}
