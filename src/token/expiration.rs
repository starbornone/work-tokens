use std::time::{SystemTime, UNIX_EPOCH};

/// Structure representing a token with an optional expiration time.
#[derive(Debug, Clone)]
pub struct Token {
    pub amount: u64,
    pub expiration_time: Option<u64>, // Timestamp when the token expires
}

impl Token {
    /// Checks if the token has expired.
    ///
    /// # Returns
    /// * `true` if the token has expired, `false` otherwise.
    pub fn has_expired(&self) -> bool {
        if let Some(expiration) = self.expiration_time {
            let current_time = get_current_timestamp();
            return current_time > expiration;
        }
        false
    }
}

/// Helper function to get the current timestamp (in seconds since UNIX epoch).
fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_expiration() {
        let token = Token {
            amount: 100,
            expiration_time: Some(get_current_timestamp() - 100), // Expired token
        };

        assert!(token.has_expired());

        let non_expired_token = Token {
            amount: 100,
            expiration_time: Some(get_current_timestamp() + 1000), // Still valid
        };

        assert!(!non_expired_token.has_expired());
    }
}
