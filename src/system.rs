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

        self.block_number
    }

    //Incrementing the block number, increases the block number by one
    pub fn inc_block_number(&mut self){
       self.block_number= self.block_number.checked_add(1).unwrap();

    }
    //Increments the nonce of an account, for keep tracking of the amount of transactions
    pub fn inc_nance(&mut self,who: &String){
        unimplemented!()
    }

}

#[cfg!(test)]
mod test{
    use std::alloc::System;

    #[test]
    fn init_system(){
        let system = super::Pallet::new();
        assert_eq!(system.block_number, 0);
    }
    #[test]
    fn inc_block_number(){
        let mut system = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number, 1);
    }


}