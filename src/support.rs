

pub struct Block<Header, Extrinsic>{
    pub header: Header,

    pub extrinsic: Vec<Extrinsic>,

}



pub struct Header<BlockNumber>{

    pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller,Call>{
    pub caller: Call,
    pub call: Call,


}

pub type DispatchResult = Result<(),&'static str>;


pub trait Dispatch{

    type Call;

    type Caller;

    fn dispatch(&mut self,caller: Self::Caller,call: Self::Call)->DispatchResult;
}