pub struct Block<Header, Extrinsic> {
    // The header contains metadata about the block
    pub header: Header,
    // The extrinsics represent the state transitions
    // to be executed in this block
    pub extrinsics: Vec<Extrinsic>,
}

pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

pub type DispatchResult = Result<(), &'static str>;

pub trait Dispatch {
    type Caller;
    type Call;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
