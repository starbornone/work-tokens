use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// Application configuration settings.
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub consensus: ConsensusSettings,
    pub database: DatabaseSettings,
    pub wallet: WalletSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConsensusSettings {
    pub difficulty: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub connection_string: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WalletSettings {
    pub wallet_dir: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            consensus: ConsensusSettings { difficulty: 4 },
            database: DatabaseSettings {
                connection_string: "sqlite://blockchain.db".to_string(),
            },
            wallet: WalletSettings {
                wallet_dir: "./wallets".to_string(),
            },
        }
    }
}

impl Settings {
    /// Loads settings from `config/settings.toml` (optional) and `APP_*` environment variables.
    pub fn new() -> Result<Self, ConfigError> {
        let defaults = Settings::default();

        let settings = Config::builder()
            .set_default("consensus.difficulty", defaults.consensus.difficulty as i64)?
            .set_default(
                "database.connection_string",
                defaults.database.connection_string.clone(),
            )?
            .set_default("wallet.wallet_dir", defaults.wallet.wallet_dir.clone())?
            .add_source(File::with_name("config/settings").required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        settings.try_deserialize()
    }
}
