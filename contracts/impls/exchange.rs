pub use crate::traits::exchange::*;
use crate::{
    impls::exchange_data::ExchangeStorage,
    traits::usd_token::UsdTokenRef,
};
use brush::contracts::psp22::*;
use ink_prelude::vec::Vec;

impl<T: ExchangeStorage> Exchange for T {
    default fn native_to_token(&mut self) -> Result<u128, ExchangeError> {
        let caller = Self::env().caller();
        let token_reserve = UsdTokenRef::balance_of(&self.get().usd_token, caller);
        let transfered_value = Self::env().transferred_value();
        let input_reserve = Self::env()
            .balance()
            .checked_sub(transfered_value)
            .ok_or(ExchangeError::SubUnderflow)?;
        let tokens_bought = self.price(transfered_value, input_reserve, token_reserve)?;
        UsdTokenRef::transfer(&self.get().usd_token, caller, tokens_bought, Vec::<u8>::new())?;
        Ok(tokens_bought)
    }

    default fn token_to_native(&mut self, token_amount: u128) -> Result<u128, ExchangeError> {
        let caller = Self::env().caller();
        let token_reserve = UsdTokenRef::balance_of(&self.get().usd_token, caller);
        let native_bought = self.price(token_amount, token_reserve, Self::env().balance())?;
        if Self::env().transfer(caller, token_amount).is_err() {
            return Err(ExchangeError::NativeTransferFailed)
        }
        Ok(native_bought)
    }

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
        numerator.checked_div(denominator).ok_or(ExchangeError::DivByZero)
    }
}

impl From<PSP22Error> for ExchangeError {
    fn from(err: PSP22Error) -> Self {
        ExchangeError::PSP22Error(err)
    }
}
