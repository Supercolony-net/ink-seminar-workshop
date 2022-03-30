use brush::{
    declare_storage_trait,
};
use ink_storage::traits::{
    SpreadAllocate,
    SpreadLayout,
};

pub use dex_derive::ExchangeStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct ExchangeStruct {
    pub ex: bool
}

declare_storage_trait!(ExchangeStorage, ExchangeStruct);