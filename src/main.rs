use std::collections::BTreeMap;
use crate::balances::Pallet;

mod balances;

fn main() {
    println!("Hello Rust");
   // let mut pallet:Pallet = balances::Pallet::new();
    let mut map= BTreeMap::new();
    map.insert("alice",100);
    assert_eq!(map.get(&"alice"), Some(&100)); // Trying to get a value it will return sth or none //the result is an option
    // it can be an option or nothing
    assert_eq!(map.get(&"bob"), None);

    let match_value: Option<&i32> = map.get("asdfasdf");
    match match_value {
        Some(value)=> {
            println!("{}",value);
            //do sth with the value
        },
        None=> {
            //Perhaps return an error
        }
    }






}
