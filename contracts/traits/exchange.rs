use brush::{
    contracts::traits::errors::PSP22Error,
    traits::AccountId,
};

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ExchangeError {
    MulOverflow,
    AddOverflow,
    DivByZero,
    SubUnderflow,
    NativeTransferFailed,
    PSP22Error(PSP22Error),
}

#[brush::trait_definition]
pub trait Exchange {
    #[ink(message, payable)]
    fn native_to_token(&mut self) -> Result<u128, ExchangeError>;

    #[ink(message)]
    fn token_to_native(&mut self, token_amount: u128) -> Result<u128, ExchangeError>;

    fn _price(&self, input_amount: u128, input_reserve: u128, output_reserve: u128) -> Result<u128, ExchangeError>;
}

#[brush::trait_definition]
pub trait ExchangeData {
    #[ink(message)]
    fn usd_token(&self) -> AccountId;

    #[ink(message)]
    fn total_liquidity(&self) -> u128;

    #[ink(message)]
    fn liquidity(&self, from: AccountId) -> u128;

    #[ink(message, payable)]
    fn init(&mut self, token_amount: u128) -> Result<u128, ExchangeError>;
}
