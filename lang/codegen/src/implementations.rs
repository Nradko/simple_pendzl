use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::Block;

pub type IsDefault = bool;
pub type OverridenFnMap =
    HashMap<String, Vec<(String, (Box<Block>, Vec<syn::Attribute>, IsDefault))>>;

pub struct ImplArgs<'a> {
    pub map: &'a OverridenFnMap,
    pub items: &'a mut Vec<syn::Item>,
    pub imports: &'a mut HashMap<&'a str, syn::ItemUse>,
    pub overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
    pub storage_struct_name: String,
}

impl<'a> ImplArgs<'a> {
    pub fn new(
        map: &'a OverridenFnMap,
        items: &'a mut Vec<syn::Item>,
        imports: &'a mut HashMap<&'a str, syn::ItemUse>,
        overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
        storage_struct_name: String,
    ) -> Self {
        Self {
            map,
            items,
            imports,
            overriden_traits,
            storage_struct_name,
        }
    }

    fn contract_name(&self) -> proc_macro2::Ident {
        format_ident!("{}", self.storage_struct_name)
    }

    fn vec_import(&mut self) {
        let vec_import = syn::parse2::<syn::ItemUse>(quote!(
            use ink::prelude::vec::Vec;
        ))
        .expect("Should parse");
        self.imports.insert("vec", vec_import);
    }

    // fn signature_import(&mut self) {
    //     let sig_import = syn::parse2::<syn::ItemUse>(quote!(
    //         use pendzl::utils::crypto::Signature;
    //     ))
    //     .expect("Should parse");
    //     self.imports.insert("Signature", sig_import);
    // }
}

pub(crate) fn impl_psp22(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::implementation::PSP22InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::token::psp22::PSP22Internal for #storage_struct_name {
            fn _total_supply(&self) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22InternalImpl::_total_supply_impl(self)
            }

            fn _balance_of(&self, owner: &AccountId) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22InternalImpl::_balance_of_impl(self, owner)
            }

            fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22InternalImpl::_allowance_impl(self, owner, spender)
            }

            fn _update(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22InternalImpl::_update_impl(self, from, to, amount)
            }

            fn _approve(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22InternalImpl::_approve_impl(self, owner, spender, amount)
            }

            fn _decrease_allowance_from_to(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22InternalImpl::_decrease_allowance_from_to_impl(self, owner, spender, amount)
            }

            fn _increase_allowance_from_to(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error>{
                pendzl::contracts::token::psp22::implementation::PSP22InternalImpl::_increase_allowance_from_to_impl(self, owner, spender, amount)

            }
        }
    ))
    .expect("Should parse");

    let psp22_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::implementation::PSP22Impl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp22 = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::PSP22 for #storage_struct_name {
            #[ink(message)]
            fn total_supply(&self) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22Impl::total_supply_impl(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22Impl::balance_of_impl(self, owner)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
                pendzl::contracts::token::psp22::implementation::PSP22Impl::allowance_impl(self, owner, spender)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22Impl::transfer_impl(self, to, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22Impl::transfer_from_impl(self, from, to, value, data)
            }

            #[ink(message)]
            fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22Impl::approve_impl(self, spender, value)
            }

            #[ink(message)]
            fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22Impl::increase_allowance_impl(self, spender, delta_value)
            }

            #[ink(message)]
            fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::token::psp22::implementation::PSP22Impl::decrease_allowance_impl(self, spender, delta_value)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::token::psp22::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::token::psp22::implementation::Data as PSP22Data;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PSP22", import);
    impl_args.imports.insert("PSP22Data", import_data);
    impl_args.vec_import();

    override_functions(
        "PSP22InternalImpl",
        &mut internal,
        impl_args.map,
    );
    override_functions("PSP22", &mut psp22, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(psp22_impl));
    impl_args.items.push(syn::Item::Impl(psp22));
}


pub(crate) fn impl_psp22_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::extensions::metadata::implementation::PSP22MetadataImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::token::psp22::extensions::metadata::PSP22Metadata for #storage_struct_name {
            #[ink(message)]
            fn token_name(&self) -> Option<String> {
                pendzl::contracts::token::psp22::extensions::metadata::implementation::PSP22MetadataImpl::token_name_impl(self)
            }

            #[ink(message)]
            fn token_symbol(&self) -> Option<String> {
                pendzl::contracts::token::psp22::extensions::metadata::implementation::PSP22MetadataImpl::token_symbol_impl(self)
            }

            #[ink(message)]
            fn token_decimals(&self) -> u8 {
                pendzl::contracts::token::psp22::extensions::metadata::implementation::PSP22MetadataImpl::token_decimals_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::token::psp22::extensions::metadata::*;
    ))
    .expect("Should parse");
    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::token::psp22::extensions::metadata::implementation::Data as PSP22MetadataData;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP22Metadata", import);
    impl_args.imports.insert("PSP22MetadataData", import_data);
    impl_args.vec_import();

    override_functions("PSP22Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(metadata_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_ownable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::ownable::implementation::OwnableInternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::ownable::OwnableInternal for #storage_struct_name {
            fn _owner(&self) -> Option<AccountId>{
                pendzl::contracts::access::ownable::implementation::OwnableInternalImpl::_owner_impl(self)
            }
            fn _update_owner(&mut self, owner: &Option<AccountId>){
                pendzl::contracts::access::ownable::implementation::OwnableInternalImpl::_update_owner_impl(self, owner);

            }
            fn _only_owner(&self) -> Result<(), OwnableError> {
                pendzl::contracts::access::ownable::implementation::OwnableInternalImpl::_only_owner_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let ownable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::ownable::implementation::OwnableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut ownable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::ownable::Ownable for #storage_struct_name {
            #[ink(message)]
            fn owner(&self) -> Option<AccountId> {
                pendzl::contracts::access::ownable::implementation::OwnableImpl::owner_impl(self)
            }

            #[ink(message)]
            fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
                pendzl::contracts::access::ownable::implementation::OwnableImpl::renounce_ownership_impl(self)
            }

            #[ink(message)]
            fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
                pendzl::contracts::access::ownable::implementation::OwnableImpl::transfer_ownership_impl(self, new_owner)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::access::ownable::*;
    ))
    .expect("Should parse");
    
    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::access::ownable::implementation::Data as OwnableData;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("Ownable", import);
    impl_args.imports.insert("OwnableData", import_data);

    override_functions("ownable::Internal", &mut internal, impl_args.map);
    override_functions("Ownable", &mut ownable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(ownable_impl));
    impl_args.items.push(syn::Item::Impl(ownable));
}

pub(crate) fn impl_access_control(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::access_control::implementation::AccessControlInternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::access_control::AccessControlInternal for #storage_struct_name {
            fn _default_admin() -> RoleType {
                <Self as pendzl::contracts::access::access_control::implementation::AccessControlInternalImpl>::_default_admin_impl()
            }

            fn _has_role(&self, role: RoleType, account: Option<AccountId>) -> bool{
                pendzl::contracts::access::access_control::implementation::AccessControlInternalImpl::_has_role_impl(self, role, account)
            }

            fn _grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalImpl::_grant_role_impl(self, role, account)
            }

            fn _do_revoke_role(&mut self, role: RoleType, account: Option<AccountId>)  -> Result<(), AccessControlError>  {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalImpl::_do_revoke_role_impl(self, role, account)
            }
            
            fn _get_role_admin(&self, role: RoleType) -> RoleType {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalImpl::_get_role_admin_impl(self, role)
            }

            fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalImpl::_set_role_admin_impl(self, role, new_admin);
            }

            fn _ensure_has_role(&self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlInternalImpl::_ensure_has_role_impl(self, role, account)
            }

        }
    ))
    .expect("Should parse");

    let access_control_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::access_control::implementation::AccessControlImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut access_control = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access::access_control::AccessControl for #storage_struct_name {
            #[ink(message)]
            fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool {
                pendzl::contracts::access::access_control::implementation::AccessControlImpl::has_role_impl(self, role, address)
            }

            #[ink(message)]
            fn get_role_admin(&self, role: RoleType) -> RoleType {
                pendzl::contracts::access::access_control::implementation::AccessControlImpl::get_role_admin_impl(self, role)
            }

            #[ink(message)]
            fn grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlImpl::grant_role_impl(self, role, account)
            }

            #[ink(message)]
            fn revoke_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlImpl::revoke_role_impl(self, role, account)
            }

            #[ink(message)]
            fn renounce_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access::access_control::implementation::AccessControlImpl::renounce_role_impl(self, role, account)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::access::access_control::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::access::access_control::implementation::Data as AccessControlData;
    ))
    .expect("Should parse import");
        
    impl_args.imports.insert("AccessControl", import);
    impl_args.imports.insert("AccessControlData", import_data);

    override_functions("access_control::AccessControlInternal", &mut internal, impl_args.map);
    override_functions("AccessControl", &mut access_control, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(access_control_impl));
    impl_args.items.push(syn::Item::Impl(access_control));
}

pub(crate) fn impl_pausable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::security::pausable::implementation::PausableInternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::security::pausable::PausableInternal for #storage_struct_name {
            fn _paused(&self) -> bool {
                pendzl::contracts::security::pausable::implementation::PausableInternalImpl::_paused_impl(self)
            }

            fn _pause(&mut self) -> Result<(), PausableError> {
                pendzl::contracts::security::pausable::implementation::PausableInternalImpl::_pause_impl(self)
            }

            fn _unpause(&mut self) -> Result<(), PausableError> {
                pendzl::contracts::security::pausable::implementation::PausableInternalImpl::_unpause_impl(self)
            }

            fn _ensure_paused(&self) -> Result<(), PausableError> {
                pendzl::contracts::security::pausable::implementation::PausableInternalImpl::_ensure_paused_impl(self)
            }

            fn _ensure_not_paused(&self) -> Result<(), PausableError> {
                pendzl::contracts::security::pausable::implementation::PausableInternalImpl::_ensure_not_paused_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let pausable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::security::pausable::implementation::PausableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut pausable = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::security::pausable::Pausable for #storage_struct_name {
            #[ink(message)]
            fn paused(&self) -> bool {
                pendzl::contracts::security::pausable::implementation::PausableImpl::paused_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::security::pausable::*;
    ))
    .expect("Should parse import");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::security::pausable::implementation::Data as PausableData;
    ))
    .expect("Should parse import");
    impl_args.imports.insert("Pausable", import);
    impl_args.imports.insert("PausableData", import_data);

    override_functions("pausable::Internal", &mut internal, impl_args.map);
    override_functions("Pausable", &mut pausable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(pausable_impl));
    impl_args.items.push(syn::Item::Impl(pausable));
}

fn override_functions(trait_name: &str, implementation: &mut syn::ItemImpl, map: &OverridenFnMap) {
    if let Some(overrides) = map.get(trait_name) {
        // we will find which fns we wanna override
        for (fn_name, (fn_code, attributes, is_default)) in overrides {
            for item in implementation.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        if !is_default {
                            method.block = *fn_code.clone();
                        }
                        method.attrs.append(&mut attributes.to_vec());
                    }
                }
            }
        }
    }
}
