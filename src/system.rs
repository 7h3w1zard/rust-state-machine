use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + CheckedAdd + CheckedSub + AddAssign + Copy;
    type Nonce: Zero + One + AddAssign + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    block_number: T::BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }
    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }
    // Increment the nonce of an account.This helps us keep track
    // of how many transactions each account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(who.clone(), nonce + T::Nonce::one());
    }
}

#[cfg(test)]
mod test {
    use crate::system::{self};
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system: super::Pallet<TestConfig> = system::Pallet::new();
        for _ in 1..=2026 {
            system.inc_block_number(); // 2026
        }
        let who = "alice".to_string();
        system.inc_nonce(&who); // 1
        system.inc_nonce(&who); // 2
        system.inc_nonce(&who); // 3
        assert_eq!(system.block_number(), 2026);
        assert_eq!(system.nonce.get(&who).unwrap_or(&0), &3);
    }
}
