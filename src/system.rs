use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
    /// The current block number.
    block_number: u32,
    /// A map from an account to their nonce.
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self { block_number: 0, nonce: BTreeMap::new() }
    }
    /// Get the current block number.
    pub fn block_number(&self) -> u32 {
        self.block_number
    }
    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }
    // Increment the nonce of an account.This helps us keep track
    // of how many transactions each account has made.
    pub fn inc_nonce(&mut self, who: &String) {
        if let Some(nonce) = self.nonce.get_mut(who) {
            *nonce += 1
        } else {
            self.nonce.insert(who.clone(), 1);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::system;

    #[test]
    fn init_system() {
        let mut system = system::Pallet::new();
        for _ in 1..=2026 {            
            system.inc_block_number(); // 2026
        }
        let who = "alice".to_string();
        system.inc_nonce(&who); // 1
        system.inc_nonce(&who); // 2
        system.inc_nonce(&who); // 3
        dbg!(assert_eq!(system.block_number(), 2026));
        assert_eq!(system.nonce.get(&who).unwrap_or(&0), &3);

    }
}