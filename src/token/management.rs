use crate::token::expiration::Token;
use std::collections::HashMap;

pub struct TokenManager {
    pub balances: HashMap<String, Vec<Token>>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn add_tokens(&mut self, user_id: &str, tokens: Vec<Token>) {
        self.balances
            .entry(user_id.to_string())
            .or_default()
            .extend(tokens);
    }

    /// Transfers exactly `amount` of non-expired tokens. On failure, balances are unchanged.
    /// When a selected token exceeds the remaining amount, change is returned to the sender.
    pub fn transfer_tokens(&mut self, from_user: &str, to_user: &str, amount: u64) -> bool {
        if amount == 0 {
            return false;
        }

        let from_balance = match self.balances.get(from_user) {
            Some(balance) => balance,
            None => return false,
        };

        let mut remaining = amount;
        let mut selected: Vec<(usize, Token)> = Vec::new();

        for (index, token) in from_balance.iter().enumerate() {
            if remaining == 0 {
                break;
            }
            if token.has_expired() {
                continue;
            }

            let take = token.amount.min(remaining);
            selected.push((
                index,
                Token {
                    amount: take,
                    expiration_time: token.expiration_time,
                },
            ));
            remaining -= take;
        }

        if remaining != 0 {
            return false;
        }

        // Apply removals from highest index so earlier indices stay valid.
        let mut sorted = selected;
        sorted.sort_by(|a, b| b.0.cmp(&a.0));

        let mut transferred = Vec::new();
        {
            let from_balance = self.balances.get_mut(from_user).expect("checked above");
            for (index, taken) in sorted {
                let token = &mut from_balance[index];
                if taken.amount >= token.amount {
                    transferred.push(from_balance.remove(index));
                } else {
                    token.amount -= taken.amount;
                    transferred.push(taken);
                }
            }
        }

        self.add_tokens(to_user, transferred);
        true
    }

    pub fn get_balance(&self, user_id: &str) -> u64 {
        self.balances
            .get(user_id)
            .map(|tokens| {
                tokens
                    .iter()
                    .filter(|token| !token.has_expired())
                    .map(|t| t.amount)
                    .sum()
            })
            .unwrap_or(0)
    }
}

impl Default for TokenManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_balance() {
        let mut manager = TokenManager::new();
        manager.add_tokens(
            "user_1",
            vec![Token {
                amount: 100,
                expiration_time: None,
            }],
        );

        assert_eq!(manager.get_balance("user_1"), 100);
    }

    #[test]
    fn test_transfer_tokens() {
        let mut manager = TokenManager::new();
        manager.add_tokens(
            "user_1",
            vec![Token {
                amount: 100,
                expiration_time: None,
            }],
        );

        assert!(manager.transfer_tokens("user_1", "user_2", 100));
        assert_eq!(manager.get_balance("user_2"), 100);
        assert_eq!(manager.get_balance("user_1"), 0);
    }

    #[test]
    fn test_transfer_with_change() {
        let mut manager = TokenManager::new();
        manager.add_tokens(
            "user_1",
            vec![Token {
                amount: 100,
                expiration_time: None,
            }],
        );

        assert!(manager.transfer_tokens("user_1", "user_2", 40));
        assert_eq!(manager.get_balance("user_2"), 40);
        assert_eq!(manager.get_balance("user_1"), 60);
    }

    #[test]
    fn test_failed_transfer_does_not_mutate() {
        let mut manager = TokenManager::new();
        manager.add_tokens(
            "user_1",
            vec![Token {
                amount: 30,
                expiration_time: None,
            }],
        );

        assert!(!manager.transfer_tokens("user_1", "user_2", 50));
        assert_eq!(manager.get_balance("user_1"), 30);
        assert_eq!(manager.get_balance("user_2"), 0);
    }
}
