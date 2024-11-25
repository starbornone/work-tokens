use crate::token::expiration::Token;
use std::time::{SystemTime, UNIX_EPOCH}; // Added missing imports

/// Structure representing a token issuance event.
#[derive(Debug, Clone)]
pub struct Issuance {
    pub issued_tokens: Vec<Token>,
    pub issuer: String,     // Who issued the tokens
    pub issuance_time: u64, // When the tokens were issued
}

impl Issuance {
    /// Issues a new token with a given amount and optional expiration time.
    pub fn issue_token(amount: u64, expiration_time: Option<u64>, _issuer: &str) -> Token {
        Token {
            amount,
            expiration_time,
        }
    }

    /// Issues multiple tokens at once.
    pub fn batch_issue(tokens: Vec<Token>, issuer: String) -> Self {
        Self {
            issued_tokens: tokens,
            issuer,
            issuance_time: get_current_timestamp(),
        }
    }
}

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
    fn test_single_token_issuance() {
        let token = Issuance::issue_token(100, None, "issuer_1");
        assert_eq!(token.amount, 100);
        assert!(token.expiration_time.is_none());
    }

    #[test]
    fn test_batch_issuance() {
        let tokens = vec![
            Issuance::issue_token(100, None, "issuer_1"),
            Issuance::issue_token(200, Some(get_current_timestamp() + 1000), "issuer_1"),
        ];
        let issuance = Issuance::batch_issue(tokens.clone(), "issuer_1".to_string());

        assert_eq!(issuance.issued_tokens.len(), 2);
        assert_eq!(issuance.issuer, "issuer_1");
    }
}
