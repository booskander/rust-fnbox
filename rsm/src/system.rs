use std::{collections::BTreeMap, ops::AddAssign};

use num::{
    traits::{CheckedAdd, CheckedSub, One, Zero},
    zero,
};

// if we need to change those values further in development
// it can be very useful and time saving
type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy + AddAssign,
    Nonce: Ord + Clone + Copy + Zero + One,
{
    pub fn new() -> Self {
        Self {
            block_number: zero(),
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one()
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        self.nonce.insert(who.clone(), nonce + Nonce::one());
    }

    pub fn get_nonce(&mut self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let system: super::Pallet<String, u32, u32> = super::Pallet::new();
        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let alice: String = "Alice".to_string();
        let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();

        system.inc_nonce(&alice);
        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 2);
    }
}
