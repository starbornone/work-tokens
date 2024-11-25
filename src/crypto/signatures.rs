use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

/// Generates a new Ed25519 keypair (private and public key).
///
/// # Returns
///
/// A tuple containing the keypair and the public key as a `String` for easier storage.
pub fn generate_keypair() -> (Keypair, String) {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);
    let public_key = hex::encode(keypair.public.to_bytes());
    (keypair, public_key)
}

/// Signs a message using the private key from the keypair.
///
/// # Arguments
///
/// * `message` - The message to be signed.
/// * `keypair` - The private and public keypair used for signing.
///
/// # Returns
///
/// The signature as a byte array.
pub fn sign_message(message: &str, keypair: &Keypair) -> Vec<u8> {
    let signature: Signature = keypair.sign(message.as_bytes());
    signature.to_bytes().to_vec()
}

/// Verifies a signature using the sender's public key.
///
/// # Arguments
///
/// * `message` - The original message that was signed.
/// * `signature` - The signature to verify.
/// * `public_key` - The public key of the signer.
///
/// # Returns
///
/// A boolean indicating whether the signature is valid.
pub fn verify_signature(message: &str, signature: &[u8], public_key: &PublicKey) -> bool {
    let signature = Signature::from_bytes(signature).expect("Invalid signature format");
    public_key.verify(message.as_bytes(), &signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::PUBLIC_KEY_LENGTH;

    #[test]
    fn test_key_generation() {
        let (_keypair, public_key_hex) = generate_keypair();
        assert_eq!(public_key_hex.len(), PUBLIC_KEY_LENGTH * 2); // Hex-encoded public key should be double the length of the raw public key
    }

    #[test]
    fn test_sign_and_verify() {
        let (keypair, _) = generate_keypair();
        let message = "Test message for signing";
        let signature = sign_message(message, &keypair);

        let public_key = keypair.public;
        assert!(verify_signature(message, &signature, &public_key));
    }

    #[test]
    fn test_invalid_signature_verification() {
        let (keypair, _) = generate_keypair();
        let message = "Valid message";
        let signature = sign_message(message, &keypair);

        let fake_message = "Fake message";
        let public_key = keypair.public;
        assert!(!verify_signature(fake_message, &signature, &public_key)); // Should return false
    }
}
