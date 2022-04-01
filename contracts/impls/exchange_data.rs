use crate::traits::usd_token::UsdTokenRef;
use brush::{
    declare_storage_trait,
    traits::AccountId,
};
use ink_prelude::vec::Vec;
use ink_storage::{
    traits::{
        SpreadAllocate,
        SpreadLayout,
    },
    Mapping,
};

pub use dex_derive::ExchangeStorage;

use crate::{
    impls::exchange::ExchangeData,
    traits::exchange::ExchangeError,
};
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct ExchangeStruct {
    pub usd_token: AccountId,
    pub total_liquidity: u128,
    pub liquidity: Mapping<AccountId, u128>,
}

declare_storage_trait!(ExchangeStorage, ExchangeStruct);

impl<T: ExchangeStorage> ExchangeData for T {
    fn usd_token(&self) -> AccountId {
        self.get().usd_token
    }

    fn total_liquidity(&self) -> u128 {
        self.get().total_liquidity
    }

    fn liquidity(&self, from: AccountId) -> u128 {
        self.get().liquidity.get(&from).unwrap_or(0)
    }

    fn init(&mut self, token_amount: u128) -> Result<u128, ExchangeError> {
        let caller = Self::env().caller();
        let total_liquidity = Self::env().transferred_value();
        UsdTokenRef::transfer_from(
            &self.get().usd_token,
            caller,
            Self::env().account_id(),
            token_amount,
            Vec::<u8>::new(),
        )?;
        self.get_mut().total_liquidity = total_liquidity;
        self.get_mut().liquidity.insert(caller, &total_liquidity);
        Ok(total_liquidity)
    }
}
