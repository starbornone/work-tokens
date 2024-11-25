use crate::crypto::hash::calculate_hash;
use crate::crypto::signatures::{sign_message, verify_signature};
use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a transaction in the blockchain.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    /// Unique ID of the transaction, derived from a hash of its contents.
    pub id: String,
    /// Public key of the sender (from address).
    pub from: PublicKey, // Store the actual public key, not just a string address
    /// Public key of the recipient (to address as String).
    pub to: String,
    /// Amount of tokens being transferred.
    pub amount: u64,
    /// Timestamp of when the transaction was created.
    pub timestamp: u64,
    /// Optional expiration timestamp for tokens.
    pub expiration: Option<u64>,
    /// Digital signature of the transaction, proving authenticity.
    pub signature: Option<Vec<u8>>,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(from: PublicKey, to: String, amount: u64, expiration: Option<u64>) -> Self {
        let timestamp = get_current_timestamp();
        let mut tx = Transaction {
            id: String::new(), // We'll compute this after initialization
            from,
            to,
            amount,
            timestamp,
            expiration,
            signature: None,
        };
        tx.id = tx.calculate_hash(); // Set transaction ID based on its contents
        tx
    }

    /// Calculates the hash (ID) of the transaction based on its contents.
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{:?}{}{}{}{}",
            self.from, // Since 'from' is now PublicKey, use `{:?}` for formatting
            self.to,
            self.amount,
            self.timestamp,
            self.expiration.unwrap_or(0)
        );
        calculate_hash(&data)
    }

    /// Signs the transaction with the sender's private key.
    /// The signature proves that the transaction is authorized by the sender.
    pub fn sign(&mut self, private_key: &ed25519_dalek::Keypair) {
        let message = self.calculate_hash();
        self.signature = Some(sign_message(&message, private_key));
    }

    /// Verifies that the transaction is properly signed by the sender.
    pub fn verify_signature(&self) -> bool {
        if let Some(signature) = &self.signature {
            let message = self.calculate_hash();
            verify_signature(&message, signature, &self.from) // Use 'from' public key directly
        } else {
            false
        }
    }

    /// Validates the transaction by ensuring it has all required fields,
    /// and that it is signed and the signature is valid.
    pub fn validate(&self) -> bool {
        if self.amount == 0 {
            return false; // Invalid if no amount is transferred
        }
        if self.to.is_empty() {
            return false; // Invalid if no recipient is specified
        }
        self.verify_signature() // Call verify_signature with the from public key stored in the transaction
    }
}

/// Helper function to get the current timestamp in seconds since UNIX epoch.
fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
