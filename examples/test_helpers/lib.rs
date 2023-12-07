#[macro_export]
macro_rules! address_of {
    ($account:ident) => {
        ink_e2e::account_id(ink_e2e::AccountKeyring::$account)
    };
}

#[macro_export]
macro_rules! balance_of {
    ($client:ident, $address:ident, $account:ident) => {{
        let _msg = build_message::<ContractRef>($address.clone())
            .call(|contract| contract.balance_of(address_of!($account)));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! owner_of {
    ($client:ident, $contract:ident, $id:expr) => {{
        $client
            .call(&ink_e2e::alice(), &contract.owner_of(id))
            .dry_run()
            .await
            .expect("owner of dry failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! balance_of_37 {
    ($client:ident, $address:ident, $account:ident, $token:expr) => {{
        let _msg = build_message::<ContractRef>($address.clone())
            .call(|contract| contract.balance_of(address_of!($account), $token));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! has_role {
    ($client:ident, $contract:ident, $role:expr, $account:ident) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &$contract.has_role($role, Some(address_of!($account))),
            )
            .dry_run()
            .await
            .expect("has_role dry failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! grant_role {
    ($client:ident, $contract:ident, $role:expr, $account:ident) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &$contract.grant_role($role, Some(address_of!($account))),
            )
            .submit()
            .await
            .expect("grant_role failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! revoke_role {
    ($client:ident, $contract:ident, $role:expr, $account:ident) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &$contract.revoke_role($role, Some(address_of!($account))),
            )
            .submit()
            .await
            .expect("revoke_role_failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! mint_dry_run {
    ($client:ident, $contract:ident, $signer:ident, $account:ident, $amount:ident) => {{
        $client
            .call(
                &ink_e2e::$signer(),
                contract.mint(address_of!($account), $amount),
            )
            .dry_run()
            .await
            .expect("mint_dry_run failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! mint {
    ($client:ident, $address:ident, $signer:ident, $account:ident, $amount:ident) => {{
        $client
            .call(
                &ink_e2e::$signer(),
                contract.mint(address_of!($account), $amount),
            )
            .submit()
            .await
            .expect("mint failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_role_member_count {
    ($client:ident, $contract:ident, $role:expr) => {{
        $client
            .call(&ink_e2e::alice(), &contract.get_role_member_count($role))
            .dry_run()
            .await
            .expect("get_role_member_count failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_role_member {
    ($client:ident, $contract:ident, $role:expr, $index:expr) => {{
        $client
            .call(&ink_e2e::alice(), &contract.get_role_member($role, $index))
            .dry_run()
            .await
            .expect("get_role_member failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_shares {
    ($client:ident, $contract:ident, $user:ident) => {{
        $client
            .call(&ink_e2e::alice(), &contract.shares(address_of!($user)))
            .dry_run()
            .await
            .expect("get_shares failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! method_call {
    ($client:ident, $address:ident, $method:ident) => {{
        let _msg = build_message::<ContractRef>($address.clone()).call(|contract| contract.$method());
        $client
            .call(&ink_e2e::alice(), _msg, )
            .await
            .expect("method_call failed")
            .return_value()
    }};
    ($client:ident, $address:ident, $signer:ident, $method:ident) => {{
        let _msg = build_message::<ContractRef>($address.clone()).call(|contract| contract.$method());
        $client
            .call(&ink_e2e::$signer(), _msg, )
            .await
            .expect("method_call failed")
            .return_value()
    }};
    ($client:ident, $address:ident, $method:ident($($args:expr),*)) => {{
        let _msg = build_message::<ContractRef>($address.clone()).call(|contract| contract.$method($($args),*));
        $client
            .call(&ink_e2e::alice(), _msg, )
            .await
            .expect("method_call failed")
            .return_value()
    }};
    ($client:ident, $address:ident, $signer:ident, $method:ident($($args:expr),*)) => {{
        let _msg = build_message::<ContractRef>($address.clone()).call(|contract| contract.$method($($args),*));
        $client
            .call(&ink_e2e::$signer(), _msg, )
            .await
            .expect("method_call failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! method_call_dry_run {
    ($client:ident, $address:ident, $method:ident) => {{
        let _msg = build_message::<ContractRef>($address.clone()).call(|contract| contract.$method());
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, )
            .await
            .return_value()
    }};
    ($client:ident, $address:ident, $method:ident($($args:expr),*)) => {{
        let _msg = build_message::<ContractRef>($address.clone()).call(|contract| contract.$method($($args),*));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, )
            .await
            .return_value()
    }};
    ($client:ident, $address:ident, $signer:ident, $method:ident) => {{
        let _msg = build_message::<ContractRef>($address.clone()).call(|contract| contract.$method());
        $client
            .call_dry_run(&ink_e2e::$signer(), &_msg, )
            .await
            .return_value()
    }};
    ($client:ident, $address:ident, $signer:ident, $method:ident($($args:expr),*)) => {{
        let _msg = build_message::<ContractRef>($address.clone()).call(|contract| contract.$method($($args),*));
        $client
            .call_dry_run(&ink_e2e::$signer(), &_msg, )
            .await
            .return_value()
    }};
}
