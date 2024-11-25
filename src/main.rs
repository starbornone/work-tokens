fn main() {
    let settings = Settings::new().expect("Failed to load settings");
    println!("Consensus Difficulty: {}", settings.consensus.difficulty);

    println!("Starting Work Tokens Blockchain Node...");
}
