pub mod merkle_tree {
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

        pub fn add_balance(&mut self, user: String, balance: u64) {
            self.balances.insert(user.clone(), balance);
            let leaf_hash: String = self.hash_pair(&user, &balance.to_string());
            self.leaves.push(leaf_hash);
            self.update_root();
        }

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

        fn hash_pair(&self, left: &str, right: &str) -> String {
            let mut hasher: Sha256 = Sha256::new();
            hasher.update(left.as_bytes());
            hasher.update(right.as_bytes());
            let result: sha2::digest::generic_array::GenericArray<u8, _> = hasher.finalize();
            hex::encode(result)
        }

        pub fn get_balance(&self, user: &str) -> Option<u64> {
            self.balances.get(user).cloned()
        }

        pub fn get_root(&self) -> Option<String> {
            self.root.clone()
        }

        pub fn size_in_bytes(&self) -> usize {
            bincode::serialized_size(self).unwrap() as usize
        }

        pub fn size_in_kb(&self) -> f64 {
            self.size_in_bytes() as f64 / 1024.0
        }

        pub fn size_in_mb(&self) -> f64 {
            self.size_in_kb() / 1024.0
        }
    }
}
