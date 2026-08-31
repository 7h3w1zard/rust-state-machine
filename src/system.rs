use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
    /// The current block number.
    block_number: BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountID, Nonce>,
}

impl<AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce>
where
    AccountID: Ord + Clone,
    BlockNumber: Zero + One + CheckedAdd + CheckedSub + AddAssign + Copy,
    Nonce: Zero + One + AddAssign + Copy,
{
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }
    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one();
    }
    // Increment the nonce of an account.This helps us keep track
    // of how many transactions each account has made.
    pub fn inc_nonce(&mut self, who: &AccountID) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        self.nonce.insert(who.clone(), nonce + Nonce::one());
    }
}

#[cfg(test)]
mod test {
    use crate::system;

    #[test]
    fn init_system() {
        let mut system: super::Pallet<String, u32, u32> = system::Pallet::new();
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
