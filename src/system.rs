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

    //Return the current block number
    pub fn block_number(&self)->u32{

        unimplemented!()
    }

    //Incrementing the block number, increases the block number by one
    pub fn inc_block_number(&mut self){
        unimplemented!()

    }
    //Increments the nonce of an account, for keep tracking of the amount of transactions
    pub fn inc_nance(&mut self,who: &String){
        unimplemented!()
    }
}