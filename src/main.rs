use std::collections::BTreeMap;
use crate::balances::Pallet;

mod balances;
mod system;
pub struct RunTime{
    system: system::Pallet,
    balances: balances::Pallet

}

impl RunTime{

    fn new()-> Self{
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new()
        }
    }
}
fn main() {
    let runTime = RunTime::new();
    println!("Hello Rust");








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


