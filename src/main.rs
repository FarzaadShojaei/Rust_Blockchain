use crate::balances::Pallet;

mod balances;

fn main() {
    println!("Hello Rust");
    let mut pallet:Pallet = balances::Pallet::new();

}
