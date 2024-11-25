use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read, Write};

/// Serializes a given struct or data into a JSON string.
pub fn serialize_to_json<T: Serialize>(data: &T) -> serde_json::Result<String> {
    serde_json::to_string(data)
}

/// Deserializes a JSON string back into the original data structure.
pub fn deserialize_from_json<'a, T: Deserialize<'a>>(json_str: &'a str) -> serde_json::Result<T> {
    serde_json::from_str(json_str)
}

/// Serializes a struct into a binary format using Bincode.
pub fn serialize_to_binary<T: Serialize>(data: &T) -> bincode::Result<Vec<u8>> {
    bincode::serialize(data)
}

/// Deserializes binary data back into the original data structure using Bincode.
pub fn deserialize_from_binary<'a, T: Deserialize<'a>>(
    binary_data: &'a [u8],
) -> bincode::Result<T> {
    bincode::deserialize(binary_data)
}

/// Saves serialized data to a file.
pub fn save_to_file(file_path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(data)?;
    Ok(())
}

/// Loads serialized data from a file.
pub fn load_from_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        name: String,
        value: u32,
    }

    #[test]
    fn test_json_serialization() {
        let test_data = TestStruct {
            name: "Test".to_string(),
            value: 42,
        };

        let json_str = serialize_to_json(&test_data).unwrap();
        let deserialized_data: TestStruct = deserialize_from_json(&json_str).unwrap();

        assert_eq!(test_data, deserialized_data);
    }

    #[test]
    fn test_binary_serialization() {
        let test_data = TestStruct {
            name: "Test".to_string(),
            value: 42,
        };

        let binary_data = serialize_to_binary(&test_data).unwrap();
        let deserialized_data: TestStruct = deserialize_from_binary(&binary_data).unwrap();

        assert_eq!(test_data, deserialized_data);
    }

    #[test]
    fn test_save_and_load_file() {
        let test_data = TestStruct {
            name: "Test".to_string(),
            value: 42,
        };

        let binary_data = serialize_to_binary(&test_data).unwrap();
        let file_path = "test_data.bin";

        // Save to file
        save_to_file(file_path, &binary_data).unwrap();

        // Load from file
        let loaded_data = load_from_file(file_path).unwrap();
        let deserialized_data: TestStruct = deserialize_from_binary(&loaded_data).unwrap();

        assert_eq!(test_data, deserialized_data);

        // Clean up test file
        std::fs::remove_file(file_path).unwrap();
    }
}
