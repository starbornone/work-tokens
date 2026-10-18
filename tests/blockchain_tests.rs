use core::blockchain::block::Block;
use core::blockchain::ledger::Ledger;
use core::blockchain::merkle_tree::MerkleTree;
use core::blockchain::transaction::Transaction;
use core::consensus::proof_of_work::ProofOfWork;
use core::crypto::keys::generate_keypair;

fn signed_tx(to: &str, amount: u64) -> Transaction {
    let keypair = generate_keypair();
    let mut tx = Transaction::new(keypair.public, to.to_string(), amount, None);
    tx.sign(&keypair);
    tx
}

#[test]
fn test_block_creation() {
    let transactions = vec![signed_tx("receiver", 50), signed_tx("receiver2", 100)];
    let block = Block::new(1, "prev_hash".to_string(), transactions, 0);

    assert!(!block.hash.is_empty());
    assert_eq!(block.index, 1);
    assert_eq!(block.previous_hash, "prev_hash");
    assert_eq!(block.transactions.len(), 2);
    assert!(block.validate());
}

#[test]
fn test_ledger_initialization() {
    let ledger = Ledger::new(2);
    assert_eq!(ledger.chain.len(), 1);
    assert_eq!(ledger.chain[0].index, 0);
}

#[test]
fn test_add_mined_block() {
    let difficulty = 2;
    let mut ledger = Ledger::new(difficulty);
    let transactions = vec![signed_tx("receiver", 100)];
    let mut new_block = Block::new(
        1,
        ledger.get_latest_block().hash.clone(),
        transactions,
        0,
    );

    ProofOfWork::new(difficulty).mine_block(&mut new_block);

    assert!(ledger.add_block(new_block));
    assert_eq!(ledger.chain.len(), 2);
    assert!(ledger.validate_chain());
}

#[test]
fn test_reject_unmined_block() {
    let difficulty = 3;
    let mut ledger = Ledger::new(difficulty);
    let new_block = Block::new(
        1,
        ledger.get_latest_block().hash.clone(),
        vec![signed_tx("receiver", 100)],
        0,
    );

    // Fresh block is almost certainly below difficulty 3 without mining.
    if ProofOfWork::meets_difficulty(&new_block.hash, difficulty) {
        // Extremely unlikely; skip rather than flake.
        return;
    }

    assert!(!ledger.add_block(new_block));
}

#[test]
fn test_invalid_previous_hash() {
    let mut ledger = Ledger::new(1);
    let mut invalid_block = Block::new(1, "fake_hash".to_string(), vec![signed_tx("r", 100)], 0);
    ProofOfWork::new(1).mine_block(&mut invalid_block);

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
    let block = Block::new(1, "prev_hash".to_string(), vec![], 0);

    assert!(!block.hash.is_empty());
    assert!(block.transactions.is_empty());
    assert!(block.validate());
}

#[test]
fn test_unsigned_transaction_rejected_by_validate() {
    let keypair = generate_keypair();
    let tx = Transaction::new(keypair.public, "receiver".to_string(), 50, None);
    assert!(!tx.validate());
}

#[test]
fn test_proof_of_work_updates_hash() {
    let difficulty = 2;
    let mut block = Block::new(1, "prev_hash".to_string(), vec![], 0);
    ProofOfWork::new(difficulty).mine_block(&mut block);

    assert_eq!(block.hash, block.calculate_hash());
    assert!(ProofOfWork::meets_difficulty(&block.hash, difficulty));
}

#[test]
fn test_reject_wrong_index() {
    let difficulty = 1;
    let mut ledger = Ledger::new(difficulty);
    let mut future_block = Block::new(
        2,
        ledger.get_latest_block().hash.clone(),
        vec![signed_tx("receiver", 100)],
        0,
    );
    ProofOfWork::new(difficulty).mine_block(&mut future_block);

    assert!(!ledger.add_block(future_block));
}
