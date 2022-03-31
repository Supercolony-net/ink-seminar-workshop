use brush::{
    declare_storage_trait,
    traits::AccountId,
};
use ink_storage::{
    traits::{
        SpreadAllocate,
        SpreadLayout,
    },
    Mapping,
};

pub use dex_derive::ExchangeStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct ExchangeStruct {
    pub total_liquidity: u128,
    pub liquidity: Mapping<AccountId, u128>,
}

declare_storage_trait!(ExchangeStorage, ExchangeStruct);
