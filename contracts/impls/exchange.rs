use crate::{
    impls::exchange_data::ExchangeStorage,
    traits::exchange::*,
};

impl<T: ExchangeStorage> Exchange for T {
    default fn price(
        &self,
        input_amount: u128,
        input_reserve: u128,
        output_reserve: u128,
    ) -> Result<u128, ExchangeError> {
        let input_amount_with_fee = input_amount.checked_mul(997).ok_or(ExchangeError::MulOverflow)?;
        let numerator = input_amount_with_fee
            .checked_mul(output_reserve)
            .ok_or(ExchangeError::MulOverflow)?;
        let denominator = input_reserve
            .checked_mul(output_reserve)
            .ok_or(ExchangeError::MulOverflow)?
            .checked_add(input_amount_with_fee)
            .ok_or(ExchangeError::AddOverflow)?;
        let b = numerator.checked_div(denominator).ok_or(ExchangeError::DivByZero)?;
        Ok(b)
    }
}
