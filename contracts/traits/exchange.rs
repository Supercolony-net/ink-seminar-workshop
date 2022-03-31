#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ExchangeError {
    MulOverflow,
    AddOverflow,
    DivByZero,
}

#[brush::trait_definition]
pub trait Exchange {
    #[ink(message)]
    fn price(&self, input_amount: u128, input_reserve: u128, output_reserve: u128) -> Result<u128, ExchangeError>;
}
