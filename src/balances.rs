use std::collections::BTreeMap;
use std::fmt::Debug;
use crate::balances;
use num::traits::{CheckedAdd,CheckedSub,Zero};


#[derive(Debug)]
pub struct Pallet<AccountId,Balance> {
 pub balances: BTreeMap<AccountId, Balance>,

}
//Adding Different Implementations of Functions : using impl
//creating a pallet from outside
impl<AccountId,Balance> Pallet<AccountId,Balance>
where
AccountId: Ord + Clone,
Balance: Zero + CheckedSub + CheckedAdd + Copy,{
    pub fn new() -> Self {
        Self{
            balances: BTreeMap::new(),
        }
    }
    /// Set the balance of account `who` to some `amount`
    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
        /* Insert `amount` into the BTreeMap under `who` */

    }
    /// GET the balance of account `who` to some `amount`
    /// If the account has no stored balance, we return zero
    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    /// Transfer `amount` from one account to another
    /// This function verifies that `from` has at least `amount` balance to transfer
    /// and that no mathematical overflows occur
   pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
    let caller_balance:Balance = self.balance(&caller);
    let to_balance:Balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount).ok_or("Insufficient balance")?;

        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow When adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);



    Ok(())


    }


}
#[cfg(test)]
mod tests {
    use crate::balances;






    #[test]
fn init_balances() {
    let mut balances = balances::Pallet::new();

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

    let mut balances= super::Pallet::new();

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
        let mut balances= super::Pallet::new();
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
        let mut balances= super::Pallet::new();
        balances.set_balance(&"alice".to_string(), *&100);
        balances.set_balance(&"alice".to_string(), u128::MAX);
        let result = balances.transfer(alice.clone(),bob.clone(), 1);
        assert_eq!(result, Err("Overflow When adding to balance"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), u128::MAX);
    }
}

