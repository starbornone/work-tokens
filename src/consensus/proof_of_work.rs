use crate::blockchain::block::Block;

pub struct ProofOfWork {
    /// Number of leading zero hex digits required in the block hash.
    pub difficulty: usize,
}

impl ProofOfWork {
    pub fn new(difficulty: usize) -> Self {
        Self { difficulty }
    }

    /// Mines a block by searching for a nonce that satisfies the difficulty target.
    /// Updates `block.hash` when a valid nonce is found.
    pub fn mine_block(&self, block: &mut Block) -> bool {
        let difficulty = self.difficulty.min(64);
        let target = Self::difficulty_target(difficulty);

        loop {
            let hash = block.calculate_hash();
            if Self::is_valid_hash(&hash, &target) {
                block.hash = hash;
                return true;
            }
            block.nonce = block.nonce.wrapping_add(1);
        }
    }

    /// Returns true if `hash` meets the given difficulty.
    pub fn meets_difficulty(hash: &str, difficulty: usize) -> bool {
        let difficulty = difficulty.min(64);
        Self::is_valid_hash(hash, &Self::difficulty_target(difficulty))
    }

    fn difficulty_target(difficulty: usize) -> String {
        format!("{}{}", "0".repeat(difficulty), "f".repeat(64 - difficulty))
    }

    fn is_valid_hash(hash: &str, target: &str) -> bool {
        hash.len() == 64 && hash <= target
    }
}
