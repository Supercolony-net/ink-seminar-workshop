#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod exchange {
    use dex::impls::{
        exchange::*,
        exchange_data::*,
    };
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, ExchangeStorage)]
    pub struct ExchangeContract {
        #[ExchangeStorageField]
        exchange: ExchangeStruct,
    }

    impl Exchange for ExchangeContract {}

    impl ExchangeData for ExchangeContract {}

    impl ExchangeContract {
        #[ink(constructor)]
        pub fn new(usd_token: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut ExchangeContract| {
                instance.exchange.usd_token = usd_token;
                instance.exchange.total_liquidity = 0;
            })
        }
    }
}
