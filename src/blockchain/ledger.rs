use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::crypto::hash::calculate_hash;

/// Represents the blockchain ledger, which consists of a chain of blocks.
pub struct Ledger {
    /// The list of blocks, representing the entire blockchain.
    pub chain: Vec<Block>,
}

impl Ledger {
    /// Creates a new ledger with the genesis block (the first block in the blockchain).
    pub fn new() -> Self {
        let genesis_block = Ledger::create_genesis_block();
        Ledger {
            chain: vec![genesis_block],
        }
    }

    /// Creates the genesis block, which is the first block in the blockchain.
    fn create_genesis_block() -> Block {
        let genesis_transactions = vec![];
        Block::new(0, String::from("0"), genesis_transactions, 0)
    }

    /// Returns the latest block in the blockchain.
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Blockchain is empty")
    }

    /// Adds a new block to the ledger after validating it.
    pub fn add_block(&mut self, mut new_block: Block) -> bool {
        // Validate the new block before adding it to the ledger
        let latest_block = self.get_latest_block();

        if new_block.previous_hash != latest_block.hash {
            println!("Error: New block's previous hash does not match the latest block's hash.");
            return false;
        }

        new_block.hash = new_block.calculate_hash();

        // Check that the new block is valid
        if !new_block.validate() {
            println!("Error: New block is invalid.");
            return false;
        }

        self.chain.push(new_block);
        true
    }

    /// Validates the integrity of the entire blockchain.
    /// This ensures that each block links properly to the previous block and that all hashes are valid.
    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if the current block's previous hash matches the previous block's hash
            if current_block.previous_hash != previous_block.hash {
                println!("Error: Block {}'s previous hash does not match the hash of the previous block.", current_block.index);
                return false;
            }

            // Check if the current block's hash is valid
            if current_block.hash != current_block.calculate_hash() {
                println!("Error: Block {}'s hash is invalid.", current_block.index);
                return false;
            }
        }
        true
    }

    /// Retrieves a block by its index in the blockchain.
    pub fn get_block_by_index(&self, index: u64) -> Option<&Block> {
        self.chain.iter().find(|&block| block.index == index)
    }

    /// Retrieves a block by its hash.
    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.chain.iter().find(|&block| block.hash == hash)
    }

    /// Retrieves all transactions across the entire blockchain.
    pub fn get_all_transactions(&self) -> Vec<&Transaction> {
        self.chain
            .iter()
            .flat_map(|block| block.transactions.iter())
            .collect()
    }
}
