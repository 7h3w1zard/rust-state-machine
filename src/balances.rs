use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    /// Set the balance o an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }
    /// Get the balnce of an account `who`.
    /// If the account has no stored balance, we return zero(0).
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }
    /// Transfer `amount` from one account to another.
    /// This function verifies thet `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let from_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_from_balance = from_balance
            .checked_sub(amount)
            .ok_or("Not enought funds.")?;
        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Not enough space in recipient storage for this funds")?;

        self.set_balance(&caller, new_from_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::balances;

    #[test]
    fn init_balances() {
        let mut balances = balances::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 666);
        assert_eq!(balances.balance(&"alice".to_string()), 666);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = balances::Pallet::new();
        let from = "alice".to_string();
        let to = "bob".to_string();
        let _set_balance = balances.set_balance(&from, 666);
        let _fail_transfer = balances.transfer(&from, &to, 667);
        assert_eq!(balances.balance(&"alice".to_string()), 666);
        assert_eq!(balances.balance(&"bob".to_string()), 0);

        let _success_transfer = balances.transfer(&from, &to, 555);
        assert_eq!(balances.balance(&"alice".to_string()), 111);
        assert_eq!(balances.balance(&"bob".to_string()), 555);
    }
}
