use sha2::{Digest, Sha256};

/// Calculates the SHA-256 hash of the given input string.
///
/// # Arguments
///
/// * `input` - A string slice that holds the data to be hashed.
///
/// # Returns
///
/// A `String` representing the hexadecimal form of the SHA-256 hash.
pub fn calculate_hash(input: &str) -> String {
    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Write input data
    hasher.update(input.as_bytes());

    // Finalize the hash and convert the output to a byte array
    let result = hasher.finalize();

    // Convert the byte array to a hexadecimal string
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hash() {
        let input = "test";
        let hash = calculate_hash(input);

        // Expected SHA-256 hash of "test" is known.
        assert_eq!(
            hash,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    fn test_empty_string_hash() {
        let input = "";
        let hash = calculate_hash(input);

        // Expected SHA-256 hash of an empty string is known.
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
