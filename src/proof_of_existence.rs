use core::fmt::Debug;
use std::collections::BTreeMap;
use std::thread::sleep;
use crate::support::DispatchResult;

pub trait Config: crate::system::Config{


    type Content: Debug + Ord;
}


#[derive(Debug)]
pub struct Pallet<T:Config>{

claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T:Config> Pallet<T>{
    pub fn new()-> Self{
       Self{
           claims: BTreeMap::new(),
       }
    }
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId>{
        self.claims.get(claim)
    }
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content)-> DispatchResult{
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None=>{
                self.claims.insert(claim,caller);
                Ok(())
            }
        }

    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult{
      let claim_owner= self.get_claim(&claim).ok_or("Claim does not exist");
        if claim_owner != Ok(&caller) {
            return Err("Caller is not the owner of the claim")
        }
        self.claims.remove(&claim);

        Ok(())
    }

}

#[cfg(test)]

mod test{
    struct TestConfig{}

    impl super::Config for TestConfig {
        type Content =&'static str;

    }
    impl crate::system::Config for TestConfig{
        type AccountId= &'static str;
        type BlockNumber= u32;

        type Nance= u32;
    }

    #[test]
    fn basic_proof_of_existence(){

        let mut poe= super::Pallet::<TestConfig>::new();
        let _  = poe.create_claim("alice","my_document");

        assert_eq!(poe.get_claim(&"my_document"),Some(&"alice"));

        let res = poe.revoke_claim("alice","my_document");
        assert_eq!(res,Err("Caller is not the owner of the claim"));

        let res = poe.create_claim("bob","my_document");
        assert_eq!(res,Err("Claim already exists"));

        let res= poe.revoke_claim("alice","non existent");
        assert_eq!(res,Err("Claim does not exist"));

        let res = poe.revoke_claim("alice","my_document");
        assert_eq!(res,Ok(()));
        assert_eq!(poe.get_claim(&"my_document"),None);

      /*
        TODO:
        Create an end to end test verifying the basic functionality of this pallet

       */
    }
}