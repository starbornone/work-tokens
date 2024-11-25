use crate::blockchain::merkle_tree::MerkleTree;
use crate::blockchain::transaction::Transaction;
use crate::crypto::hash::calculate_hash;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a block in the blockchain.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    /// Block index (height in the blockchain).
    pub index: u64,
    /// Timestamp of when the block was created.
    pub timestamp: u64,
    /// Hash of the previous block in the blockchain.
    pub previous_hash: String,
    /// The Merkle root, which is a hash of all the transactions in this block.
    pub merkle_root: String,
    /// The hash of this block (calculated based on its contents).
    pub hash: String,
    /// The list of transactions included in this block.
    pub transactions: Vec<Transaction>,
    /// A nonce used for the consensus algorithm (e.g., Proof-of-Work).
    pub nonce: u64,
}

impl Block {
    /// Creates a new block.
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        nonce: u64,
    ) -> Self {
        let timestamp = get_current_timestamp();
        let merkle_tree =
            MerkleTree::new(transactions.iter().map(|tx| tx.calculate_hash()).collect());
        let merkle_root = merkle_tree.root.clone().unwrap_or_default();

        let mut block = Block {
            index,
            timestamp,
            previous_hash: previous_hash.clone(),
            merkle_root,
            hash: String::new(),
            transactions,
            nonce,
        };

        // Calculate the block hash based on its contents
        block.hash = block.calculate_hash();
        block
    }

    /// Calculates the hash of the block based on its contents.
    pub fn calculate_hash(&self) -> String {
        let block_contents = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.previous_hash,
            self.merkle_root,
            self.nonce,
            self.transactions.len()
        );
        calculate_hash(&block_contents)
    }

    /// Validates the block by comparing its calculated hash with its stored hash.
    pub fn validate(&self) -> bool {
        self.hash == self.calculate_hash()
    }
}

/// Helper function to get the current timestamp in seconds since the UNIX epoch.
fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
