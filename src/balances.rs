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
}