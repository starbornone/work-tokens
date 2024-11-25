#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::block::Block;
    use crate::blockchain::ledger::Ledger;
    use crate::blockchain::merkle_tree::MerkleTree;
    use crate::blockchain::transaction::Transaction;
    use crate::crypto::hash::calculate_hash;

    #[test]
    fn test_block_creation() {
        let transactions = vec![
            Transaction::new("sender".to_string(), "receiver".to_string(), 50, None),
            Transaction::new("sender2".to_string(), "receiver2".to_string(), 100, None),
        ];
        let block = Block::new(1, "prev_hash".to_string(), transactions, 0);

        assert!(!block.hash.is_empty());
        assert_eq!(block.index, 1);
        assert_eq!(block.previous_hash, "prev_hash");
        assert_eq!(block.transactions.len(), 2);
        println!("Block Hash: {}", block.hash);
    }

    #[test]
    fn test_block_validation() {
        let transactions = vec![Transaction::new(
            "sender".to_string(),
            "receiver".to_string(),
            50,
            None,
        )];
        let block = Block::new(1, "prev_hash".to_string(), transactions, 0);
        assert!(block.validate());
    }

    #[test]
    fn test_ledger_initialization() {
        let ledger = Ledger::new();
        assert_eq!(ledger.chain.len(), 1);
        assert_eq!(ledger.chain[0].index, 0);
    }

    #[test]
    fn test_add_block() {
        let mut ledger = Ledger::new();
        let transactions = vec![Transaction::new(
            "sender".to_string(),
            "receiver".to_string(),
            100,
            None,
        )];
        let new_block = Block::new(1, ledger.get_latest_block().hash.clone(), transactions, 0);

        assert!(ledger.add_block(new_block));
        assert_eq!(ledger.chain.len(), 2);
    }

    #[test]
    fn test_validate_chain() {
        let mut ledger = Ledger::new();
        let transactions = vec![Transaction::new(
            "sender".to_string(),
            "receiver".to_string(),
            100,
            None,
        )];
        let new_block = Block::new(1, ledger.get_latest_block().hash.clone(), transactions, 0);
        ledger.add_block(new_block);

        assert!(ledger.validate_chain());
    }

    #[test]
    fn test_invalid_block_addition() {
        let mut ledger = Ledger::new();
        let transactions = vec![Transaction::new(
            "sender".to_string(),
            "receiver".to_string(),
            100,
            None,
        )];
        let mut invalid_block = Block::new(1, "fake_hash".to_string(), transactions, 0);
        invalid_block.hash = invalid_block.calculate_hash();

        assert!(!ledger.add_block(invalid_block));
    }

    #[test]
    fn test_merkle_tree_creation() {
        let transactions = vec![
            "tx1_hash".to_string(),
            "tx2_hash".to_string(),
            "tx3_hash".to_string(),
        ];

        let merkle_tree = MerkleTree::new(transactions);
        assert!(merkle_tree.root.is_some());
        println!("Merkle Root: {:?}", merkle_tree.root);
    }

    #[test]
    fn test_merkle_tree_verification() {
        let transactions = vec!["tx1_hash".to_string(), "tx2_hash".to_string()];

        let merkle_tree = MerkleTree::new(transactions.clone());
        let proof = vec![transactions[1].clone()];
        let is_valid =
            merkle_tree.verify_proof(&transactions[0], &proof, merkle_tree.root.as_ref().unwrap());

        assert!(is_valid);
    }

    #[test]
    fn test_block_with_no_transactions() {
        let transactions: Vec<Transaction> = vec![];
        let block = Block::new(1, "prev_hash".to_string(), transactions, 0);

        assert!(!block.hash.is_empty());
        assert_eq!(block.transactions.len(), 0);
        assert!(
            block.merkle_root.is_empty()
                || block.merkle_root == "some_expected_value_for_empty_root"
        );
    }

    #[test]
    fn test_genesis_block_structure() {
        let ledger = Ledger::new();
        let genesis_block = &ledger.chain[0];

        assert_eq!(genesis_block.index, 0);
        assert_eq!(genesis_block.previous_hash, "0");
        assert!(genesis_block.hash != "");
        assert_eq!(genesis_block.transactions.len(), 0);
    }

    #[test]
    fn test_invalid_transaction_in_block() {
        let invalid_transaction = Transaction::new(
            "invalid_sender".to_string(),
            "receiver".to_string(),
            50,
            None,
        );
        let block = Block::new(1, "prev_hash".to_string(), vec![invalid_transaction], 0);

        assert!(
            !block.validate(),
            "Block with invalid transaction should not be valid"
        );
    }

    #[test]
    fn test_proof_of_work() {
        let mut block = Block::new(1, "prev_hash".to_string(), vec![], 0);
        let difficulty = 4; // Example difficulty level
        block.mine(difficulty);

        let leading_zeros = "0".repeat(difficulty);
        assert!(
            block.hash.starts_with(&leading_zeros),
            "Block hash does not meet the difficulty requirement"
        );
    }

    #[test]
    fn test_adding_future_block() {
        let mut ledger = Ledger::new();
        let transactions = vec![Transaction::new(
            "sender".to_string(),
            "receiver".to_string(),
            100,
            None,
        )];

        // Simulate a future block with a timestamp much higher than the current time.
        let mut future_block =
            Block::new(2, ledger.get_latest_block().hash.clone(), transactions, 0);
        future_block.timestamp += 1000000; // Add an unrealistic timestamp

        assert!(
            !ledger.add_block(future_block),
            "Should not allow adding a block with a future timestamp"
        );
    }
}
