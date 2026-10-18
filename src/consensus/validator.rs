use crate::blockchain::block::Block;
use crate::blockchain::merkle_tree::MerkleTree;
use crate::blockchain::transaction::Transaction;
use crate::consensus::proof_of_work::ProofOfWork;

pub struct Validator;

impl Validator {
    /// Validates a block: PoW difficulty, merkle root, hash consistency, and all transactions.
    pub fn validate_block(block: &Block, difficulty: usize) -> bool {
        if !block.validate() {
            return false;
        }

        let expected_root = MerkleTree::root_from_transactions(&block.transactions);
        if block.merkle_root != expected_root {
            return false;
        }

        if !ProofOfWork::meets_difficulty(&block.hash, difficulty) {
            return false;
        }

        for transaction in &block.transactions {
            if !Self::validate_transaction(transaction) {
                return false;
            }
        }

        true
    }

    /// Validates a transaction signature and basic field rules.
    pub fn validate_transaction(transaction: &Transaction) -> bool {
        transaction.validate()
    }
}
