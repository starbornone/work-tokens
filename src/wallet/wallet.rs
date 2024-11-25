use crate::blockchain::transaction::Transaction;
use crate::crypto::generate_keypair;
use crate::crypto::keys::{load_keypair_from_private, save_keypair};
use ed25519_dalek::{Keypair, Signature, Signer}; // Ensured correct imports
use std::path::Path;

pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    /// Creates a new wallet by generating a new keypair.
    pub fn new() -> Self {
        let keypair = generate_keypair();
        Wallet { keypair }
    }

    /// Saves the wallet's keypair to disk.
    ///
    /// # Arguments
    /// * `private_key_path` - The path where the private key will be saved.
    /// * `public_key_path` - The path where the public key will be saved.
    pub fn save(&self, private_key_path: &str, public_key_path: &str) {
        save_keypair(&self.keypair, private_key_path, public_key_path)
            .expect("Failed to save keypair");
    }

    /// Loads a wallet from the private key stored on disk.
    ///
    /// # Arguments
    /// * `private_key_path` - The path to the private key file.
    ///
    /// # Returns
    /// A `Wallet` instance with the loaded keypair.
    pub fn load_from_file(private_key_path: &str) -> Self {
        let keypair = load_keypair_from_private(private_key_path).expect("Failed to load keypair");
        Wallet { keypair }
    }

    /// Signs a transaction using the wallet's private key.
    ///
    /// # Arguments
    /// * `transaction` - The transaction to sign.
    ///
    /// # Returns
    /// A `Signature` for the transaction.
    pub fn sign_transaction(&self, transaction: &Transaction) -> Signature {
        let message = transaction.calculate_hash();
        // Use Keypair's sign method to sign the transaction hash
        self.keypair.sign(message.as_bytes())
    }

    /// Gets the wallet's public key.
    ///
    /// # Returns
    /// A reference to the wallet's public key.
    pub fn get_public_key(&self) -> &ed25519_dalek::PublicKey {
        &self.keypair.public
    }
}
