pub use crate::traits::exchange::*;
use crate::{
    impls::exchange_data::ExchangeStorage,
    traits::usd_token::UsdTokenRef,
};
use brush::contracts::psp22::*;
use ink_env::CallFlags;
use ink_prelude::vec::Vec;

impl<T: ExchangeStorage> Exchange for T {
    default fn native_to_token(&mut self) -> Result<u128, ExchangeError> {
        let token_reserve = UsdTokenRef::balance_of(&self.get().usd_token, Self::env().account_id());
        let transfered_value = Self::env().transferred_value();
        let input_reserve = Self::env()
            .balance()
            .checked_sub(transfered_value)
            .ok_or(ExchangeError::SubUnderflow)?;
        let tokens_bought = self.price(transfered_value, input_reserve, token_reserve)?;
        UsdTokenRef::transfer(
            &self.get().usd_token,
            Self::env().caller(),
            tokens_bought,
            Vec::<u8>::new(),
        )?;
        Ok(tokens_bought)
    }

    default fn token_to_native(&mut self, token_amount: u128) -> Result<u128, ExchangeError> {
        let token_reserve = UsdTokenRef::balance_of(&self.get().usd_token, Self::env().account_id());
        let native_bought = self.price(token_amount, token_reserve, Self::env().balance())?;
        if Self::env().transfer(Self::env().caller(), token_amount).is_err() {
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

    default fn deposit(&mut self) -> Result<u128, ExchangeError> {
        let caller = Self::env().caller();
        let transfered_balance = Self::env().transferred_value();
        let native_reserve = Self::env()
            .balance()
            .checked_sub(transfered_balance)
            .ok_or(ExchangeError::SubUnderflow)?;
        let token_reserve = UsdTokenRef::balance_of(&self.get().usd_token, Self::env().account_id());
        let token_amount = transfered_balance
            .checked_mul(token_reserve)
            .ok_or(ExchangeError::MulOverflow)?
            .checked_div(native_reserve)
            .ok_or(ExchangeError::DivByZero)?
            .checked_add(1)
            .ok_or(ExchangeError::AddOverflow)?;
        let liquidity_minted = transfered_balance
            .checked_mul(self.get().total_liquidity)
            .ok_or(ExchangeError::MulOverflow)?
            .checked_div(native_reserve)
            .ok_or(ExchangeError::DivByZero)?;
        let caller_liquidity = self
            .get()
            .liquidity
            .get(caller)
            .unwrap_or(0)
            .checked_add(liquidity_minted)
            .ok_or(ExchangeError::AddOverflow)?;
        self.get_mut().liquidity.insert(caller, &caller_liquidity);
        self.get_mut().total_liquidity = self
            .get()
            .total_liquidity
            .checked_add(liquidity_minted)
            .ok_or(ExchangeError::AddOverflow)?;
        UsdTokenRef::transfer_from_builder(
            &self.get().usd_token,
            caller,
            Self::env().account_id(),
            token_amount,
            Vec::<u8>::new(),
        )
        .call_flags(CallFlags::default().set_allow_reentry(true))
        .fire()
        .unwrap()?;
        Ok(liquidity_minted)
    }

    default fn withdraw(&mut self, amount: u128) -> Result<(u128, u128), ExchangeError> {
        let caller = Self::env().caller();
        let token_reserve = UsdTokenRef::balance_of(&self.get().usd_token, Self::env().account_id());
        let total_liquidity = self.get().total_liquidity;
        let native_amount = amount
            .checked_mul(Self::env().balance())
            .ok_or(ExchangeError::MulOverflow)?
            .checked_div(total_liquidity)
            .ok_or(ExchangeError::DivByZero)?;
        let token_amount = amount
            .checked_mul(token_reserve)
            .ok_or(ExchangeError::MulOverflow)?
            .checked_div(total_liquidity)
            .ok_or(ExchangeError::DivByZero)?;
        let caller_liquidity = self
            .get()
            .liquidity
            .get(caller)
            .unwrap_or(0)
            .checked_sub(native_amount)
            .ok_or(ExchangeError::SubUnderflow)?;
        self.get_mut().liquidity.insert(caller, &caller_liquidity);
        self.get_mut().total_liquidity = total_liquidity
            .checked_sub(native_amount)
            .ok_or(ExchangeError::SubUnderflow)?;
        Self::env()
            .transfer(caller, token_amount)
            .map_err(|_| ExchangeError::NativeTransferFailed)?;
        Ok((native_amount, token_amount))
    }
}

impl From<PSP22Error> for ExchangeError {
    fn from(err: PSP22Error) -> Self {
        ExchangeError::PSP22Error(err)
    }
}
