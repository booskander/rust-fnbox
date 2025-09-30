use std::collections::BTreeMap;

// if we need to change those values further in development
// it can be very useful and time saving
type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(), nonce + 1);
    }

    pub fn get_nonce(&mut self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {
    use crate::system::Pallet;
    #[test]
    fn init_system() {
        let system: Pallet = super::Pallet::new();
        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_nonce() {
        let alice: String = "Alice".to_string();
        let mut system: Pallet = super::Pallet::new();

        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 1);
    }
}
