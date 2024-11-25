use ed25519_dalek::{Keypair, PublicKey, SecretKey, SIGNATURE_LENGTH};
use rand::rngs::OsRng;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Generates a new Ed25519 keypair (public and private keys).
///
/// # Returns
///
/// * `Keypair` - A tuple containing the public and private key pair.
pub fn generate_keypair() -> Keypair {
    Keypair::generate(&mut OsRng)
}

/// Saves the keypair (private and public keys) to a file.
///
/// # Arguments
///
/// * `keypair` - The keypair to be saved.
/// * `private_key_path` - The file path where the private key will be stored.
/// * `public_key_path` - The file path where the public key will be stored.
pub fn save_keypair(
    keypair: &Keypair,
    private_key_path: &str,
    public_key_path: &str,
) -> io::Result<()> {
    // Save the private key
    let private_key_bytes = keypair.secret.as_bytes();
    let mut private_file = fs::File::create(private_key_path)?;
    private_file.write_all(private_key_bytes)?;

    // Save the public key
    let public_key_bytes = keypair.public.as_bytes();
    let mut public_file = fs::File::create(public_key_path)?;
    public_file.write_all(public_key_bytes)?;

    Ok(())
}

/// Loads a private key from a file and generates the corresponding Keypair.
///
/// # Arguments
///
/// * `private_key_path` - The file path to load the private key from.
///
/// # Returns
///
/// * `Keypair` - The generated keypair from the private key.
pub fn load_keypair_from_private(private_key_path: &str) -> io::Result<Keypair> {
    let private_key_bytes = fs::read(private_key_path)?;

    let secret_key = SecretKey::from_bytes(&private_key_bytes).expect("Invalid private key file");

    let public_key: PublicKey = (&secret_key).into();

    Ok(Keypair {
        secret: secret_key,
        public: public_key,
    })
}

/// Loads a public key from a file.
///
/// # Arguments
///
/// * `public_key_path` - The file path to load the public key from.
///
/// # Returns
///
/// * `PublicKey` - The public key loaded from the file.
pub fn load_public_key(public_key_path: &str) -> io::Result<PublicKey> {
    let public_key_bytes = fs::read(public_key_path)?;

    let public_key = PublicKey::from_bytes(&public_key_bytes).expect("Invalid public key file");

    Ok(public_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_keypair_generation() {
        let keypair = generate_keypair();
        assert_eq!(keypair.secret.as_bytes().len(), 32);
        assert_eq!(keypair.public.as_bytes().len(), 32);
    }

    #[test]
    fn test_save_and_load_keypair() {
        let keypair = generate_keypair();
        let private_key_path = "test_private.key";
        let public_key_path = "test_public.key";

        save_keypair(&keypair, private_key_path, public_key_path).expect("Failed to save keys");

        let loaded_keypair =
            load_keypair_from_private(private_key_path).expect("Failed to load private key");

        // Verify that the loaded keypair matches the original
        assert_eq!(keypair.secret.as_bytes(), loaded_keypair.secret.as_bytes());
        assert_eq!(keypair.public.as_bytes(), loaded_keypair.public.as_bytes());

        // Clean up test files
        remove_file(private_key_path).unwrap();
        remove_file(public_key_path).unwrap();
    }

    #[test]
    fn test_load_public_key() {
        let keypair = generate_keypair();
        let public_key_path = "test_public.key";

        save_keypair(&keypair, "unused_private.key", public_key_path).expect("Failed to save keys");

        let loaded_public_key =
            load_public_key(public_key_path).expect("Failed to load public key");

        // Verify that the loaded public key matches the original
        assert_eq!(keypair.public.as_bytes(), loaded_public_key.as_bytes());

        // Clean up test files
        remove_file(public_key_path).unwrap();
    }
}
