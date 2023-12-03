#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, AccessControl)]
#[ink::contract]
pub mod my_access_control {

    // use pendzl::contracts::access::access_control::Internal;
    use pendzl::contracts::token::psp22::*;
    use pendzl::contracts::{
        access::access_control::RoleType,
        token::psp22::extensions::{burnable::PSP22Burnable, mintable::PSP22Mintable},
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
        #[storage_field]
        access: AccessControlData,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

    impl PSP22Burnable for Contract {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._ensure_has_role(MINTER, Some(self.env().caller()))?;
            self._update(Some(&account), None, &amount)
        }
    }

    impl PSP22Mintable for Contract {
        #[ink(message)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._ensure_has_role(MINTER, Some(self.env().caller()))?;
            self._update(None, Some(&account), &amount)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            instance
                ._grant_role(Self::_default_admin(), Some(caller))
                .expect("caller should become admin");
            // // We grant minter role to caller in constructor, so he can mint/burn tokens
            instance
                ._grant_role(MINTER, Some(caller))
                .expect("Should grant MINTER role");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        use pendzl::contracts::access::access_control::DEFAULT_ADMIN_ROLE;
        use test_helpers::{address_of, grant_role, has_role, mint, mint_dry_run, revoke_role};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn only_minter_role_is_allowed_to_mint(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, Bob), false);

            assert!(matches!(
                mint_dry_run!(client, address, bob, Bob, 1),
                Err(_)
            ));

            assert_eq!(grant_role!(client, address, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, Bob), true);

            assert_eq!(mint!(client, address, bob, Bob, 1), Ok(()));

            let balance_of = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(balance_of, 1);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_initial_roles_to_default_signer(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, Alice), true);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, Alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_grant_initial_roles_for_random_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, Bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, Bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, Bob), false);

            assert_eq!(grant_role!(client, address, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, Bob), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_change_old_roles_after_grant_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, Bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, Bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, Alice), true);
            assert_eq!(has_role!(client, address, MINTER, Alice), true);

            assert_eq!(grant_role!(client, address, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, Bob), true);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, Bob), false);
            assert_eq!(has_role!(client, address, DEFAULT_ADMIN_ROLE, Alice), true);
            assert_eq!(has_role!(client, address, MINTER, Alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_revoke_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, Bob), false);

            assert_eq!(grant_role!(client, address, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, address, MINTER, Bob), true);

            let revoke_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.revoke_role(MINTER, Some(address_of!(Bob))));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(revoke_role, Ok(()));

            assert_eq!(has_role!(client, address, MINTER, Bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_renounce_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(has_role!(client, address, MINTER, Alice), true);

            let renounce_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_role(MINTER, Some(address_of!(Alice))));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(renounce_role, Ok(()));

            assert_eq!(has_role!(client, address, MINTER, Alice), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_grant_or_revoke_not_by_admin_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(grant_role!(client, address, MINTER, Bob), Ok(()));

            let grant_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.grant_role(MINTER, Some(address_of!(Charlie))));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(grant_role, Err(_)));

            let revoke_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.revoke_role(MINTER, Some(address_of!(Charlie))));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(revoke_role, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_renounce_not_self_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(grant_role!(client, address, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, Bob), true);

            let renounce_role = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.renounce_role(MINTER, Some(address_of!(Bob))));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(renounce_role, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_burn_if_no_minter_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_access_control", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(grant_role!(client, address, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, Bob), true);

            assert_eq!(mint!(client, address, bob, Bob, 1), Ok(()));

            let balance_of = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address.clone()));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(balance_of, 1);

            assert_eq!(revoke_role!(client, address, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, address, MINTER, Bob), false);

            let burn = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(Bob), 1));
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(burn, Err(_)));

            Ok(())
        }
    }
}
