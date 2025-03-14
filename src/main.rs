use std::collections::BTreeMap;
use std::collections::hash_set::SymmetricDifference;
use crate::balances::Pallet;
use crate::types::AccountId;

pub mod balances;
pub mod system;

 mod types{
   pub type AccountId=String;
  pub  type Balance=u128;
 pub    type BlockNumber=u32;
   pub  type Nance =u32;

}



impl system::Config for Runtime{
  type AccountId = types::AccountId;
   type BlockNumber = types::BlockNumber;
   type Nance = types::Nance;
}

impl balances::Config for Runtime{

    type Balance = types::Balance;


}

#[derive(Debug)]
pub struct Runtime{
    system: system::Pallet<Runtime>,
    balances: Pallet<Runtime>

}

impl Runtime{

    fn new()-> Self{
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new()
        }
    }
}
fn main() {
    let mut run_time = Runtime::new();
    println!("Hello Rust");
    let alice ="alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    run_time.balances.set_balance(&alice, 100);

    run_time.system.inc_block_number();


    assert_eq!(run_time.system.block_number(), 1);

    run_time.system.inc_nance(&alice);

    let _ = run_time.balances.transfer(alice.clone(), charlie.clone(), 20).map_err(|e|println!("Error: {:?}", e));

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


