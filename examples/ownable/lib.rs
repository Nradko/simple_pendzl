// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// Contract Summary:
/// An PSP22 contract with ownable module.
/// A creator of the contract becomes an owner.
/// Owner is allowed to mint and burn PSP22 tokens.
// ########################################################
// inject PSP22 trait's default implementation (PSP22DefaultImpl & PSP22InternalDefaultImpl)
// and Ownable trait's default implementation (OwnableDefaultImpl & OwnableInternalDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
#[pendzl::implementation(PSP22, Ownable)]
#[ink::contract]
pub mod ownable {
    use pendzl::contracts::psp22::{
        burnable::PSP22Burnable, mintable::PSP22Mintable,
    };

    #[ink(storage)]
    // derive explained below
    #[derive(StorageFieldGetter)]
    pub struct Contract {
        // apply the storage_field attribute so it's accessible via `self.data::<PSP22>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP22Data is a struct that implements PSP22Storage - required by PSP22InternalDefaultImpl trait
        // note it's not strictly required by PSP22 trait - just the default implementation
        // name of the field is arbitrary
        psp22: PSP22Data,
        #[storage_field]
        // OwnableData is a struct that implements OwnableStorage - required by OwnableInternalDefaultImpl trait
        // note it's not strictly required by Ownable trait - just the default implementation
        // name of the field is arbitrary
        ownable: OwnableData,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Contract::default();
            // use _update_owner to set the owner to the caller from OwnableInternal (implemented by OwnableDefaultImpl)
            instance._update_owner(&Some(Self::env().caller()));
            instance
        }
    }

    // implement PSP22Burnable for Contract
    impl PSP22Burnable for Contract {
        #[ink(message)]
        fn burn(
            &mut self,
            account: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            // use _only_owner to ensure only the owner can burn from OwnableInternal (implemented by OwnableDefaultImpl)
            self._only_owner()?;
            // use _update to update the balance from PSP22Internal (implemented by PSP22InternalDefaultImpl)
            self._update(Some(&account), None, &amount)
        }
    }

    impl PSP22Mintable for Contract {
        #[ink(message)]
        fn mint(
            &mut self,
            account: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            // use _only_owner to ensure only the owner can burn from OwnableInternal (implemented by OwnableDefaultImpl)
            self._only_owner()?;
            // use _update to update the balance from PSP22Internal (implemented by PSP22InternalDefaultImpl)
            self._update(None, Some(&account), &amount)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        use ink_e2e::{
            account_id, alice,
            AccountKeyring::{Alice, Bob},
            ContractsBackend,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn owner_is_by_default_contract_deployer(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_owner_is_allowed_to_mint(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            let mint_res = client
                .call(&alice(), &contract.mint(account_id(Bob), 1))
                .submit()
                .await
                .expect("mint failed")
                .return_value();

            assert!(matches!(mint_res, Ok(())));

            Ok(())
        }
    }
}
