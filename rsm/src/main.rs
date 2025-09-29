mod balances;
mod system;

// this is where we bundle all the modules
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
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
    }
}
fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let john = "john".to_string();

    runtime.register_block(&alice, 100);

    assert_eq!(runtime.system.block_number(), 1)
}
