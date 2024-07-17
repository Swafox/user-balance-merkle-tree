use sha2::{Digest, Sha256};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    balances: HashMap<String, u64>,
    root: Option<String>,
    leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree {
            balances: HashMap::new(),
            root: None,
            leaves: Vec::new(),
        }
    }

    /// Adds a balance for a user and updates the Merkle tree.
    ///
    /// # Arguments
    ///
    /// * `user` - A String that holds the name of the user
    /// * `balance` - The balance to be added for the user
    pub fn add_balance(&mut self, user: String, balance: u64) {
        self.balances.insert(user.clone(), balance);
        let leaf_hash: String = self.hash_pair(&user, &balance.to_string());
        self.leaves.push(leaf_hash);
        self.update_root();
    }

    /// Updates the Merkle root based on the current leaves.
    fn update_root(&mut self) {
        if self.leaves.is_empty() {
            self.root = None;
            return;
        }

        let mut current_level: Vec<String> = self.leaves.clone();

        while current_level.len() > 1 {
            let mut next_level: Vec<String> = Vec::new();
            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    let hash: String = self.hash_pair(&chunk[0], &chunk[1]);
                    next_level.push(hash);
                } else {
                    next_level.push(chunk[0].clone());
                }
            }
            current_level = next_level;
        }

        self.root = Some(current_level[0].clone());
    }

    /// Hashes two strings together using SHA256.
    ///
    /// # Arguments
    ///
    /// * `left` - The first string to hash
    /// * `right` - The second string to hash
    fn hash_pair(&self, left: &str, right: &str) -> String {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(left.as_bytes());
        hasher.update(right.as_bytes());
        let result: sha2::digest::generic_array::GenericArray<u8, _> = hasher.finalize();
        hex::encode(result)
    }

    /// Retrieves the balance for a given user.
    ///
    /// # Arguments
    ///
    /// * `user` - A string slice that holds the name of the user
    ///
    /// # Returns
    ///
    /// An Option containing the balance if the user exists, or None if not found.
    pub fn get_balance(&self, user: &str) -> Option<u64> {
        self.balances.get(user).cloned()
    }

    /// Retrieves the current Merkle root.
    ///
    /// # Returns
    ///
    /// An Option containing the Merkle root as a String, or None if the tree is empty.
    pub fn get_root(&self) -> Option<String> {
        self.root.clone()
    }

    /// Calculates the size of the MerkleTree in bytes.
    ///
    /// # Returns
    ///
    /// The size in bytes.
    pub fn size_in_bytes(&self) -> usize {
        bincode::serialize(self).unwrap().len()
    }

    /// Calculates the size of the MerkleTree in kilobytes.
    ///
    /// # Returns
    ///
    /// The size in KB.
    pub fn size_in_kb(&self) -> f64 {
        self.size_in_bytes() as f64 / 1024.0
    }

    /// Calculates the size of the MerkleTree in megabytes.
    ///
    /// # Returns
    ///
    /// The size in MB.
    pub fn size_in_mb(&self) -> f64 {
        self.size_in_kb() / 1024.0
    }
}
