use std::collections::BTreeMap;
use std::ops::AddAssign;
use num::traits::{CheckedAdd, CheckedSub, One, Zero};

pub trait Config{
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nance: Zero + One + Copy;
    //Hello System
}

#[derive(Debug)]
pub struct Pallet<T:Config> {
    block_number:T::BlockNumber,
    nance:BTreeMap<T::AccountId,T::Nance>

}

impl<T:Config>Pallet<T>



{


    pub fn new() -> Self {
        Self{
            block_number:T::BlockNumber::zero(),
            nance: BTreeMap::new(),
        }
    }

    //Return the current block number
    pub fn block_number(&self) -> T::BlockNumber {

        self.block_number
    }

    //Incrementing the block number, increases the block number by one
    pub fn inc_block_number(&mut self){
       self.block_number += T::BlockNumber::one();

    }
    //Increments the nonce of an account, for keep tracking of the amount of transactions
    pub fn inc_nance(&mut self,who: &T::AccountId){
        let nance = *self.nance.get(who).unwrap_or(&T::Nance::zero());


        self.nance.insert(who.clone(), nance+T::Nance::one());



    }

    pub fn get_nance(&self, who: &T::AccountId) -> T::Nance {
        //let default = &0;
        *self.nance.get(who).unwrap_or(&T::Nance::zero())
    }

}

#[cfg(test)]
mod test{
    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId= String;
        type BlockNumber = u32;
        type Nance = u32;
        
    }
    use std::alloc::System;
    use crate::balances::Pallet;

    #[test]
    fn init_system(){
        let system:super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(system.block_number, 0);
    }
    #[test]
    fn inc_block_number(){
        let mut system:super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number, 1);
    }



    #[test]
    fn inc_nance(){
        let alice:String = String::from("alice");
        let mut system:super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_nance(&alice.clone());
        assert_eq!(system.get_nance(&alice), 1);
    }


}