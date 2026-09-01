use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance
/// each account has in this state machine.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // A simple storage mapping from accounts to their balances.
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    /// Set the balance o an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }
    /// Get the balnce of an account `who`.
    /// If the account has no stored balance, we return zero(0).
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
    /// Transfer `amount` from one account to another.
    /// This function verifies thet `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let from_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_from_balance = from_balance
            .checked_sub(&amount)
            .ok_or("Not enought funds.")?;
        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Not enough space in recipient storage for this funds")?;

        self.set_balance(&caller, new_from_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

// A public enum which desribes the calls we want
// to expose to the dispatcher.
// We should expect that the caller of each call will be provided
// by the dispatcher, and not included as a parameter of the call.
pub enum Call<T: Config> {
    Transfer {
        to: T::AccountId,
        amount: T::Balance,
    },
}

/// Implementation of the dispatch logic, mapping from `BalancesCall`
/// to the approriate underlying function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(&caller, &to, amount)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{balances, system};
    struct TestConfig;

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances = balances::Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        assert_eq!(balances.balance(&alice), 0);
        balances.set_balance(&alice, 666);
        assert_eq!(balances.balance(&alice), 666);
        assert_eq!(balances.balance(&bob), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = balances::Pallet::<TestConfig>::new();
        let from = "alice".to_string();
        let to = "bob".to_string();
        let _set_balance = balances.set_balance(&from, 666);
        let _fail_transfer = balances.transfer(&from, &to, 667);
        assert_eq!(balances.balance(&from), 666);
        assert_eq!(balances.balance(&to), 0);

        let _success_transfer = balances.transfer(&from, &to, 555);
        assert_eq!(balances.balance(&from), 111);
        assert_eq!(balances.balance(&to), 555);
    }
}
