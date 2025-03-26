use std::collections::BTreeMap;
use std::collections::hash_set::SymmetricDifference;
use crate::balances::Pallet;
use crate::support::{Dispatch, DispatchResult};
use crate::types::{AccountId, Balance};


pub mod balances;
pub mod system;
mod support;
mod proof_of_existence;

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime{
    system: system::Pallet<Runtime>,
    balances: Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>,

}

impl system::Config for Runtime{
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}



impl balances::Config for Runtime{

    type Balance = types::Balance;


}

impl proof_of_existence::Config for Runtime{
    //   type AccountId = types::AccountId;
    type Content = types::Content;
}

mod types{
    use crate::support;

    pub type AccountId=String;
  pub  type Balance=u128;
 pub    type BlockNumber=u32;
   pub  type Nonce =u32;

    pub type Header= support::Header<BlockNumber>;
    pub type Extrinsic = support::Extrinsic<AccountId,crate::RuntimeCall>;
    pub type Block= support::Block<Header,Extrinsic>;

    pub type Content = &'static str;

    /* TODO: Define a concrete `Extrinsitc` type using `AccountId` and `Runtime Call`.  */
    /* TODO: Define a concrete `Header` type using `BlockNumber` */
    /* TODO: Define a concrete `Block` type using `Header` and `Extrinsic` */

}












fn main() {
    let mut run_time = Runtime::new();
    println!("Hello Rust");
    let alice ="alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    run_time.balances.set_balance(&alice,100);

    let block_1 = types::Block{
        header: support::Header{block_number: 1},
        extrinsics: vec![
            support::Extrinsic{
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {to: bob.clone(), amount:30})
            },
            support::Extrinsic{
                caller: alice,
                call: RuntimeCall::balances(balances::Call::transfer{to: charlie.clone(), amount:20})
            },


        ],


    };

    let block_2 = types::Block{
        header: support::Header{block_number: 2},
        extrinsics: vec![
            support::Extrinsic{
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim{claim: "my_document"})
            },
            support::Extrinsic{
               caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim{claim: "bob's document"})
            },


        ],


    };



    run_time.execute_block(block_1).expect("wrong block execution");
    run_time.execute_block(block_2).expect("wrong block execution");



    println!("{:#?}", run_time);









}

/*
    let mut map= BTreeMap::new();
    //Pattern Matching
    map.insert("alice",100);
    assert_eq!(map.get(&"alice").unwrap_or(&0),&100); // Trying to get a value it will return sth or none //the result is an option
    // it can be an option or nothing
    //if unwrap finds no value, it will return zero
    assert_eq!(map.get(&"bob"), None);

    let match_value: Option<&i32> = map.get(&"alice");
    match match_value {
        Some(value)=> {
            println!("{}",value);
            //do sth with the value
        },
        None=> {
            //Perhaps return an error
            println!("no value");
        }
    }

 */


