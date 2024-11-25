use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use rusqlite::{params, Connection, Result};

/// Struct to manage database connections and operations.
pub struct Database {
    connection: Connection,
}

impl Database {
    /// Initializes a new database connection and sets up the necessary tables.
    pub fn new(db_path: &str) -> Result<Self> {
        let connection = Connection::open(db_path)?;

        // Set up the tables if they don't already exist
        connection.execute(
            "CREATE TABLE IF NOT EXISTS blocks (
                id INTEGER PRIMARY KEY,
                block_hash TEXT NOT NULL,
                previous_hash TEXT NOT NULL,
                data TEXT NOT NULL
            )",
            [],
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY,
                block_hash TEXT NOT NULL,
                transaction_data TEXT NOT NULL,
                FOREIGN KEY(block_hash) REFERENCES blocks(block_hash)
            )",
            [],
        )?;

        Ok(Self { connection })
    }

    /// Persists a block to the database.
    pub fn save_block(&self, block: &Block) -> Result<()> {
        let block_data = serde_json::to_string(block).expect("Failed to serialize block");
        self.connection.execute(
            "INSERT INTO blocks (block_hash, previous_hash, data) VALUES (?1, ?2, ?3)",
            params![block.hash, block.previous_hash, block_data],
        )?;
        Ok(())
    }

    /// Retrieves a block by its hash from the database.
    pub fn get_block(&self, block_hash: &str) -> Result<Block> {
        let mut stmt = self
            .connection
            .prepare("SELECT data FROM blocks WHERE block_hash = ?1")?;
        let block_data: String = stmt.query_row([block_hash], |row| row.get(0))?;

        let block: Block = serde_json::from_str(&block_data).expect("Failed to deserialize block");
        Ok(block)
    }

    /// Persists a transaction to the database.
    pub fn save_transaction(&self, block_hash: &str, transaction: &Transaction) -> Result<()> {
        let transaction_data =
            serde_json::to_string(transaction).expect("Failed to serialize transaction");
        self.connection.execute(
            "INSERT INTO transactions (block_hash, transaction_data) VALUES (?1, ?2)",
            params![block_hash, transaction_data],
        )?;
        Ok(())
    }

    /// Retrieves all transactions for a given block.
    pub fn get_transactions_for_block(&self, block_hash: &str) -> Result<Vec<Transaction>> {
        let mut stmt = self
            .connection
            .prepare("SELECT transaction_data FROM transactions WHERE block_hash = ?1")?;
        let transaction_iter = stmt.query_map([block_hash], |row| row.get(0))?;

        let mut transactions = Vec::new();
        for transaction_data in transaction_iter {
            let transaction_data: String = transaction_data?; // Unwrap result here
            let transaction: Transaction = serde_json::from_str(&transaction_data)
                .map_err(|_e| rusqlite::Error::InvalidQuery)?; // Map serde_json::Error to rusqlite::Error::InvalidQuery
            transactions.push(transaction);
        }

        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::block::Block;
    use crate::blockchain::transaction::Transaction;

    #[test]
    fn test_save_and_get_block() {
        let db = Database::new(":memory:").expect("Failed to create database");
        let block = Block::new(1, "prev_hash".to_string(), vec![], 0);
        db.save_block(&block).expect("Failed to save block");

        let loaded_block = db.get_block(&block.hash).expect("Failed to retrieve block");
        assert_eq!(block.hash, loaded_block.hash);
    }

    #[test]
    fn test_save_and_get_transactions() {
        let db = Database::new(":memory:").expect("Failed to create database");
        let block = Block::new(1, "prev_hash".to_string(), vec![], 0);
        db.save_block(&block).expect("Failed to save block");

        // Generate a keypair for the sender's public key
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);

        // Create a transaction using the sender's public key
        let transaction = Transaction::new(keypair.public, "receiver".to_string(), 100, None);

        // Save the transaction in the database
        db.save_transaction(&block.hash, &transaction)
            .expect("Failed to save transaction");

        // Retrieve the transactions from the block
        let transactions = db
            .get_transactions_for_block(&block.hash)
            .expect("Failed to retrieve transactions");

        // Assert that the transaction is correct
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].amount, 100);
    }
}
