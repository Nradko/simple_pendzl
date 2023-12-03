// SPDX-License-Identifier: MIT

pub use super::*;

use ink::prelude::string::String;
use pendzl::traits::Storage;
#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Data {
    #[lazy]
    pub name: Option<String>,
    #[lazy]
    pub symbol: Option<String>,
    #[lazy]
    pub decimals: u8,
}

pub trait PSP22MetadataImpl: Storage<Data> {
    fn token_name_impl(&self) -> Option<String> {
        self.data().name.get_or_default()
    }

    fn token_symbol_impl(&self) -> Option<String> {
        self.data().symbol.get_or_default()
    }

    fn token_decimals_impl(&self) -> u8 {
        self.data().decimals.get_or_default()
    }
}
