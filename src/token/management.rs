use crate::token::expiration::Token;
use std::collections::HashMap;

pub struct TokenManager {
    pub balances: HashMap<String, Vec<Token>>, // Maps user IDs to their tokens
}

impl TokenManager {
    /// Creates a new TokenManager instance.
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    /// Adds tokens to a user's balance.
    pub fn add_tokens(&mut self, user_id: &str, tokens: Vec<Token>) {
        let user_balance = self
            .balances
            .entry(user_id.to_string())
            .or_insert(Vec::new());
        user_balance.extend(tokens);
    }

    /// Transfers tokens between users.
    pub fn transfer_tokens(&mut self, from_user: &str, to_user: &str, amount: u64) -> bool {
        if let Some(from_balance) = self.balances.get_mut(from_user) {
            let mut transferred_amount = 0;
            let mut transferred_tokens = Vec::new();

            // Transfer tokens until the requested amount is reached
            from_balance.retain(|token| {
                if transferred_amount < amount && !token.has_expired() {
                    transferred_amount += token.amount;
                    transferred_tokens.push(token.clone());
                    false // Remove from `from_user`'s balance
                } else {
                    true
                }
            });

            // If the total amount was transferred, update recipient's balance
            if transferred_amount >= amount {
                self.add_tokens(to_user, transferred_tokens);
                return true;
            }
        }
        false
    }

    /// Gets the user's total balance of valid (non-expired) tokens.
    pub fn get_balance(&self, user_id: &str) -> u64 {
        if let Some(tokens) = self.balances.get(user_id) {
            tokens
                .iter()
                .filter(|token| !token.has_expired())
                .map(|t| t.amount)
                .sum()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_balance() {
        let mut manager = TokenManager::new();
        let tokens = vec![Token {
            amount: 100,
            expiration_time: None,
        }];
        manager.add_tokens("user_1", tokens);

        assert_eq!(manager.get_balance("user_1"), 100);
    }

    #[test]
    fn test_transfer_tokens() {
        let mut manager = TokenManager::new();
        let tokens = vec![Token {
            amount: 100,
            expiration_time: None,
        }];
        manager.add_tokens("user_1", tokens);

        let success = manager.transfer_tokens("user_1", "user_2", 100);
        assert!(success);
        assert_eq!(manager.get_balance("user_2"), 100);
        assert_eq!(manager.get_balance("user_1"), 0);
    }
}
