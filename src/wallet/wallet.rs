use crate::blockchain::transaction::Transaction;
use crate::crypto::generate_keypair;
use crate::crypto::keys::{load_keypair_from_private, save_keypair};
use ed25519_dalek::{Keypair, Signature, Signer};

pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            keypair: generate_keypair(),
        }
    }

    pub fn save(&self, private_key_path: &str, public_key_path: &str) {
        save_keypair(&self.keypair, private_key_path, public_key_path)
            .expect("Failed to save keypair");
    }

    pub fn load_from_file(private_key_path: &str) -> Self {
        let keypair = load_keypair_from_private(private_key_path).expect("Failed to load keypair");
        Wallet { keypair }
    }

    pub fn sign_transaction(&self, transaction: &Transaction) -> Signature {
        let message = transaction.calculate_hash();
        self.keypair.sign(message.as_bytes())
    }

    pub fn get_public_key(&self) -> &ed25519_dalek::PublicKey {
        &self.keypair.public
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}
