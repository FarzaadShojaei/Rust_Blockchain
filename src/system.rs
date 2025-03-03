use std::collections::BTreeMap;
use std::ops::AddAssign;
use num::traits::{CheckedAdd, CheckedSub, One, Zero};

type AccountId=String;
type BlockNumber=u32;
type Nance =u32;
#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nance>{
    
    block_number:BlockNumber,
    nance:BTreeMap<AccountId,Nance>


    
    
}

impl <AccountId, BlockNumber, Nance>Pallet<AccountId, BlockNumber, Nance>
where
    AccountId: Ord+ Clone,
    BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy + AddAssign,
    Nance: Ord+Clone+Copy

{


    pub fn new() -> Self {
        Self{
            block_number:BlockNumber::zero(),
            nance:BTreeMap::new()
        }
    }

    //Return the current block number
    pub fn block_number(&self) -> BlockNumber {

        self.block_number;
    }

    //Incrementing the block number, increases the block number by one
    pub fn inc_block_number(&mut self){
       self.block_number += BlockNumber::one();

    }
    //Increments the nonce of an account, for keep tracking of the amount of transactions
    pub fn inc_nance(&mut self,who: &AccountId){
        let nance: &u32 = self.nance.get(who).unwrap_or(&0);
        let new_nance: u32 = nance +1 ;

        self.nance.insert(who.clone(), nance+1);



    }

    pub fn get_nance(&self, who: &AccountId) -> &u32 {
        let default = &0;
        self.nance.get(who).unwrap_or(default)
    }

}

#[cfg(test)]
mod test{
    use std::alloc::System;
    use crate::balances::Pallet;

    #[test]
    fn init_system(){
        let system = super::Pallet::new();
        assert_eq!(system.block_number, 0);
    }
    #[test]
    fn inc_block_number(){
        let mut system = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number, 1);
    }



    #[test]
    fn inc_nance(){
        let alice:String = String::from("alice");
        let mut system = super::Pallet::new();
        system.inc_nance(&alice.clone());
        assert_eq!(system.get_nance(&alice).unwrap(), 1);
    }


}