use crate::crypto::hash::calculate_hash;
use serde::{Deserialize, Serialize};

/// Represents a Merkle Tree in the blockchain, which ensures transaction integrity.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerkleTree {
    pub root: Option<String>,
    pub transactions: Vec<String>, // Stores the transaction hashes
}

impl MerkleTree {
    /// Constructs a new Merkle Tree from a list of transactions.
    pub fn new(transaction_hashes: Vec<String>) -> Self {
        let root = if transaction_hashes.is_empty() {
            None
        } else {
            Some(build_merkle_root(transaction_hashes.clone()))
        };

        MerkleTree {
            root,
            transactions: transaction_hashes,
        }
    }

    /// Verifies if a transaction hash is part of the Merkle Tree by comparing it to the root.
    pub fn verify_proof(&self, tx_hash: &String, proof: &Vec<String>, root: &String) -> bool {
        let mut current_hash = tx_hash.clone();

        for sibling_hash in proof {
            current_hash = combine_and_hash(&current_hash, sibling_hash);
        }

        &current_hash == root
    }
}

/// Combines two hashes and computes their parent hash.
fn combine_and_hash(left: &String, right: &String) -> String {
    let combined = format!("{}{}", left, right);
    calculate_hash(&combined)
}

/// Builds the Merkle Root by recursively hashing the transaction hashes.
fn build_merkle_root(mut transaction_hashes: Vec<String>) -> String {
    // Base case: If only one hash is left, return it as the root
    if transaction_hashes.len() == 1 {
        return transaction_hashes[0].clone();
    }

    // If the number of transactions is odd, duplicate the last one
    if transaction_hashes.len() % 2 != 0 {
        let last_hash = transaction_hashes.last().unwrap().clone();
        transaction_hashes.push(last_hash);
    }

    let mut parent_level = vec![];

    // Combine pairs of transactions to compute their parent hashes
    for i in (0..transaction_hashes.len()).step_by(2) {
        let parent_hash = combine_and_hash(&transaction_hashes[i], &transaction_hashes[i + 1]);
        parent_level.push(parent_hash);
    }

    // Recursively build the tree by computing parent levels until we get to the root
    build_merkle_root(parent_level)
}
