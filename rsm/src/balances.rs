use std::collections::BTreeMap;

#[derive(Debug)]

pub struct Pallet {
    pub balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&mut self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {
        let mut pallet = super::Pallet::new();

        assert_eq!(pallet.get_balance(&"Alice".to_string()), 0);
    }

    #[test]
    fn success_transfer_balance() {
        let mut pallet = super::Pallet::new();

        pallet.set_balance(&"Alice".to_string(), 100);
        pallet.set_balance(&"Bob".to_string(), 100);

        let res = pallet.transfer("Alice".to_string(), "Bob".to_string(), 50);

        assert_eq!(res, Ok(()));
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 150);
    }

    #[test]
    fn fail_transfer() {
        let mut pallet = super::Pallet::new();

        pallet.set_balance(&"Alice".to_string(), 100);
        pallet.set_balance(&"Bob".to_string(), 100);

        let res = pallet.transfer("Alice".to_string(), "Bob".to_string(), 150);

        assert_eq!(res, Err("Insufficient balance"));
    }
}
