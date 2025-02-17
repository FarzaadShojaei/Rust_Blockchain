use std::collections::BTreeMap;
pub struct Pallet {
    balances: BTreeMap<String, u128>,

}
//Adding Different Implementations of Functions : using impl
//creating a pallet from outside
impl Pallet {
    pub fn new() -> Self {
        Self{
            balances: BTreeMap::new(),
        }
    }
    /// Set the balance of account `who` to some `amount`
    pub fn set_balance(&mut self, who: &String, amount: &u128) {
        /* Insert `amount` into the BTreeMap under `who` */
        unimplemented!();
    }
}


