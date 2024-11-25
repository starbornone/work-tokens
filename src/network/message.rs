use serde::{Deserialize, Serialize};

/// Enum to represent the type of message in the P2P network.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Hello,
    Block,
    Transaction,
}

/// Struct to represent a message in the P2P network.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_type: MessageType,
    pub payload: String, // This could be serialized data (e.g., block or transaction)
}

impl Message {
    /// Creates a new message.
    ///
    /// # Arguments
    /// * `message_type` - The type of the message (e.g., `Block`, `Transaction`).
    /// * `payload` - The message payload (e.g., serialized block or transaction).
    ///
    /// # Returns
    /// * `Message` - A new `Message` instance.
    pub fn new(message_type: MessageType, payload: String) -> Self {
        Message {
            message_type,
            payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let message = Message::new(MessageType::Hello, "Hello from node".to_string());
        let serialized = bincode::serialize(&message).expect("Failed to serialize message");
        let deserialized: Message =
            bincode::deserialize(&serialized).expect("Failed to deserialize message");

        assert_eq!(message.message_type, deserialized.message_type);
        assert_eq!(message.payload, deserialized.payload);
    }
}
