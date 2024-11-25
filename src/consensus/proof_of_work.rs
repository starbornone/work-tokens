use crate::blockchain::block::Block;
use crate::crypto::hash::calculate_hash;

pub struct ProofOfWork {
    pub difficulty: usize, // Difficulty level, represented by the number of leading zeros required in the hash
}

impl ProofOfWork {
    /// Mines a new block by finding a valid nonce.
    ///
    /// # Arguments
    /// * `block` - A mutable reference to the block being mined.
    ///
    /// # Returns
    /// * `bool` - Returns `true` when a valid nonce is found and the block is mined.
    pub fn mine_block(&self, block: &mut Block) -> bool {
        let target = Self::difficulty_target(self.difficulty);
        while !Self::is_valid_hash(&block.calculate_hash(), &target) {
            block.nonce += 1; // Increment the nonce to try a new hash
        }
        true
    }

    /// Creates the difficulty target based on the current difficulty level.
    ///
    /// # Arguments
    /// * `difficulty` - The number of leading zeros required in the hash.
    ///
    /// # Returns
    /// * `String` - A string representing the target (e.g., "0000...").
    fn difficulty_target(difficulty: usize) -> String {
        "0".repeat(difficulty) + &"f".repeat(64 - difficulty) // For simplicity, assume SHA-256 hash
    }

    /// Checks if the hash meets the difficulty target.
    ///
    /// # Arguments
    /// * `hash` - The hash of the block.
    /// * `target` - The difficulty target.
    ///
    /// # Returns
    /// * `bool` - Returns `true` if the hash is valid (i.e., has the required number of leading zeros).
    fn is_valid_hash(hash: &str, target: &str) -> bool {
        hash <= target
    }
}
