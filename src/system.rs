use std::collections::BTreeMap;

pub struct Pallet{
    
    block_number:u32,
    nance:BTreeMap<String,u32>


    
    
}

impl Pallet {


    pub fn new() -> Self {
        Self{
            block_number:0,
            nance:BTreeMap::new()
        }
    }
}