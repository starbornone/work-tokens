use crate::blockchain::merkle_tree::MerkleTree;
use crate::blockchain::transaction::Transaction;
use crate::crypto::hash::calculate_hash;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a block in the blockchain.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        nonce: u64,
    ) -> Self {
        let timestamp = get_current_timestamp();
        let merkle_root = MerkleTree::root_from_transactions(&transactions);

        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            merkle_root,
            hash: String::new(),
            transactions,
            nonce,
        };

        block.hash = block.calculate_hash();
        block
    }

    /// Hash of header fields used for PoW and chain linking.
    pub fn calculate_hash(&self) -> String {
        let block_contents = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, self.merkle_root, self.nonce
        );
        calculate_hash(&block_contents)
    }

    /// Checks hash consistency and that the merkle root matches the transactions.
    pub fn validate(&self) -> bool {
        if self.hash != self.calculate_hash() {
            return false;
        }

        let expected_root = MerkleTree::root_from_transactions(&self.transactions);
        self.merkle_root == expected_root
    }
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
