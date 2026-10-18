use crate::blockchain::transaction::Transaction;
use crate::crypto::hash::calculate_hash;
use serde::{Deserialize, Serialize};

/// Merkle tree over transaction hashes for block integrity.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerkleTree {
    pub root: Option<String>,
    pub transactions: Vec<String>,
}

impl MerkleTree {
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

    /// Merkle root for a transaction list. Empty blocks use the empty-string hash.
    pub fn root_from_transactions(transactions: &[Transaction]) -> String {
        let hashes: Vec<String> = transactions.iter().map(|tx| tx.calculate_hash()).collect();
        match MerkleTree::new(hashes).root {
            Some(root) => root,
            None => calculate_hash(""),
        }
    }

    /// Verifies a Merkle proof. `proof` entries are sibling hashes paired left-to-right
    /// as produced by this tree (current hash always treated as the left operand).
    pub fn verify_proof(&self, tx_hash: &str, proof: &[String], root: &str) -> bool {
        let mut current_hash = tx_hash.to_string();

        for sibling_hash in proof {
            current_hash = combine_and_hash(&current_hash, sibling_hash);
        }

        current_hash == root
    }
}

fn combine_and_hash(left: &str, right: &str) -> String {
    calculate_hash(&format!("{left}{right}"))
}

fn build_merkle_root(mut transaction_hashes: Vec<String>) -> String {
    if transaction_hashes.len() == 1 {
        return transaction_hashes[0].clone();
    }

    if transaction_hashes.len() % 2 != 0 {
        let last_hash = transaction_hashes
            .last()
            .expect("non-empty odd-length list")
            .clone();
        transaction_hashes.push(last_hash);
    }

    let mut parent_level = Vec::with_capacity(transaction_hashes.len() / 2);
    for i in (0..transaction_hashes.len()).step_by(2) {
        parent_level.push(combine_and_hash(
            &transaction_hashes[i],
            &transaction_hashes[i + 1],
        ));
    }

    build_merkle_root(parent_level)
}
