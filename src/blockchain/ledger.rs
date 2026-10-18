use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::consensus::validator::Validator;

/// In-memory chain of blocks.
pub struct Ledger {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Ledger {
    pub fn new(difficulty: usize) -> Self {
        Ledger {
            chain: vec![Self::create_genesis_block()],
            difficulty,
        }
    }

    fn create_genesis_block() -> Block {
        Block::new(0, String::from("0"), vec![], 0)
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Blockchain is empty")
    }

    /// Adds a block after checking link, height, PoW, merkle, and transactions.
    pub fn add_block(&mut self, new_block: Block) -> bool {
        let latest_block = self.get_latest_block();

        if new_block.index != latest_block.index + 1 {
            eprintln!(
                "Error: expected block index {}, got {}",
                latest_block.index + 1,
                new_block.index
            );
            return false;
        }

        if new_block.previous_hash != latest_block.hash {
            eprintln!("Error: New block's previous hash does not match the latest block's hash.");
            return false;
        }

        if !Validator::validate_block(&new_block, self.difficulty) {
            eprintln!("Error: New block failed validation.");
            return false;
        }

        self.chain.push(new_block);
        true
    }

    pub fn validate_chain(&self) -> bool {
        if self.chain.is_empty() {
            return false;
        }

        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.index != previous_block.index + 1 {
                eprintln!("Error: Block {} has non-contiguous index.", current_block.index);
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                eprintln!(
                    "Error: Block {}'s previous hash does not match the previous block.",
                    current_block.index
                );
                return false;
            }

            if !Validator::validate_block(current_block, self.difficulty) {
                eprintln!("Error: Block {} failed validation.", current_block.index);
                return false;
            }
        }
        true
    }

    pub fn get_block_by_index(&self, index: u64) -> Option<&Block> {
        self.chain.iter().find(|block| block.index == index)
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.chain.iter().find(|block| block.hash == hash)
    }

    pub fn get_all_transactions(&self) -> Vec<&Transaction> {
        self.chain
            .iter()
            .flat_map(|block| block.transactions.iter())
            .collect()
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new(4)
    }
}
