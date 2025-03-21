use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::AddAssign;
use num::One;
use crate::balances;
use num::traits::{CheckedAdd,CheckedSub,Zero};
use crate::types::{AccountId, Balance};

pub trait Config: crate::system::Config {

    type Balance: Zero + CheckedSub + CheckedAdd + Copy;

}


#[derive(Debug)]
pub struct Pallet<T:Config> {
 balances: BTreeMap<T::AccountId, T::Balance>,

}
//Adding Different Implementations of Functions : using impl
//creating a pallet from outside
impl<T:Config> Pallet<T>
{
    pub fn new() -> Self {
        Self{
            balances: BTreeMap::new(),

        }
    }
    /// Set the balance of account `who` to some `amount`
    pub fn set_balance(&mut self,who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
        /* Insert `amount` into the BTreeMap under `who` */

    }
    /// GET the balance of account `who` to some `amount`
    /// If the account has no stored balance, we return zero
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    /// Transfer `amount` from one account to another
    /// This function verifies that `from` has at least `amount` balance to transfer
    /// and that no mathematical overflows occur
   pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
    let caller_balance: <T as Config>::Balance= self.balance(&caller);
    let to_balance:<T as Config>::Balance = self.balance(&to);

        let new_caller_balance :<T as Config>::Balance = caller_balance
            .checked_sub(&amount).ok_or("Insufficient balance")?;

        let new_to_balance:<T as Config>::Balance = to_balance.checked_add(&amount).ok_or("Overflow When adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);



    Ok(())


    }


}
pub enum Call<T: Config> {
    Transfer {to: T:: AccountId, amount: T::Balance},

    //RemoveMe(core::marker::PhantomData<T>),
}
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller=T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult{
        match call{
            Call::Transfer{to,amount}=>{
                self.transfer(caller,to,amount)?

            }
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use crate::{balances, system};
    use crate::types::Balance;

    struct TestConfig;
    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber =u32;
        type Nance = u32;
    }


    impl super::Config for TestConfig{

        type Balance =u128;
    }





    #[test]
fn init_balances() {
    let mut balances:super::Pallet<TestConfig> = balances::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()),0);
    balances.set_balance(&"bob".to_string(), *&100);
    //assert_eq!(balances.balance(&"alice".to_string()),100);
    assert_eq!(balances.balance(&"bob".to_string()),100);




}




#[test]
fn failed_test(){
    assert_eq!(1,2)

}

#[test]
fn transfer_balance(){
    let alice = "alice".to_string();
    let bob = "bob".to_string();

    let mut balances:super::Pallet<TestConfig>= super::Pallet::new();

    balances.set_balance(&"alice".to_string(), *&100);

    let _ = balances.transfer(alice.clone(),bob.clone(), 90);

    assert_eq!(balances.balance(&alice), 10);
    assert_eq!(balances.balance(&bob), 90);

    /* TODO: Create a test that checks the following:
    - That `alice` cannot transfer funds she does not have
    - That `alice` can successfully transfer funds to to `bob`
    - That the balance of `alice` and `bob` is correctly updated
     */

}
    #[test]
    fn transfer_balance_insufficient(){
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let mut balances:super::Pallet<TestConfig>= super::Pallet::new();
        balances.set_balance(&"alice".to_string(), *&100);
        let result = balances.transfer(alice.clone(),bob.clone(), 90);
        assert_eq!(result, Err("Insufficient balance"));
        assert_eq!(result.is_err(), true);
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), 0);
    }
    #[test]
    fn transfer_balance_overflow(){
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let mut balances:super::Pallet<TestConfig>= super::Pallet::new();
        balances.set_balance(&"alice".to_string(), *&100);
        balances.set_balance(&"alice".to_string(), u128::MAX);
        let result = balances.transfer(alice.clone(),bob.clone(), 1);
        assert_eq!(result, Err("Overflow When adding to balance"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), u128::MAX);
    }
}

