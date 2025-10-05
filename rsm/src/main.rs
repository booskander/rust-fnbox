mod balances;
mod system;
// we can define the types outside
mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type AccountId = types::AccountId;
    type Balance = types::Balance;
}

#[derive(Debug)]

// this is where we bundle all the modules
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }

    fn register_block(&mut self, who: &String, amount: u128) {
        self.balances.set_balance(who, amount);
        self.system.inc_block_number();
        self.system.inc_nonce(who);
    }

    fn transfer_from_to(
        &mut self,
        from: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        self.balances.transfer(from.clone(), to.clone(), amount)
    }
}
fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let john = "john".to_string();

    runtime.register_block(&alice, 100);
    runtime.register_block(&bob, 100);

    let _ = runtime
        .transfer_from_to(&bob, &alice, 50)
        .map_err(|err| println!("Error: {:?}", err));

    println!("Bob has {:#?}", runtime);
}
