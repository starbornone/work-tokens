use core::blockchain::block::Block;
use core::blockchain::ledger::Ledger;
use core::blockchain::transaction::Transaction;
use core::config::Settings;
use core::consensus::proof_of_work::ProofOfWork;
use core::crypto::keys::generate_keypair;
use core::storage::Database;

fn main() {
    env_logger::init();

    let settings = Settings::new().unwrap_or_else(|err| {
        eprintln!("Failed to load settings ({err}); using defaults");
        Settings::default()
    });

    let difficulty = settings.consensus.difficulty;
    println!("Consensus difficulty: {difficulty}");
    println!("Database: {}", settings.database.connection_string);

    let mut ledger = Ledger::new(difficulty);

    let keypair = generate_keypair();
    let mut tx = Transaction::new(keypair.public, "treasury".to_string(), 50, None);
    tx.sign(&keypair);

    let mut block = Block::new(
        1,
        ledger.get_latest_block().hash.clone(),
        vec![tx],
        0,
    );

    println!("Mining block 1...");
    ProofOfWork::new(difficulty).mine_block(&mut block);
    println!("Mined hash: {}", block.hash);

    if !ledger.add_block(block.clone()) {
        eprintln!("Failed to add mined block to ledger");
        std::process::exit(1);
    }

    let db_path = settings
        .database
        .connection_string
        .strip_prefix("sqlite://")
        .unwrap_or("blockchain.db");

    match Database::new(db_path) {
        Ok(db) => {
            if let Err(err) = db.save_block(&block) {
                eprintln!("Failed to persist block: {err}");
            } else {
                println!("Persisted block to {db_path}");
            }
        }
        Err(err) => eprintln!("Failed to open database: {err}"),
    }

    println!(
        "Chain height: {} (valid={})",
        ledger.chain.len(),
        ledger.validate_chain()
    );
    println!("Work Tokens node prototype ready.");
}
