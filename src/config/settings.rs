use config::{Config, ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

/// Struct representing the application configuration settings.
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub consensus: ConsensusSettings,
    pub database: DatabaseSettings,
    pub wallet: WalletSettings,
}

/// Struct representing consensus-specific settings.
#[derive(Debug, Deserialize)]
pub struct ConsensusSettings {
    pub difficulty: usize,
}

/// Struct representing database-specific settings.
#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub connection_string: String,
}

/// Struct representing wallet-specific settings.
#[derive(Debug, Deserialize)]
pub struct WalletSettings {
    pub wallet_dir: String,
}

impl Settings {
    /// Loads the settings from configuration files and environment variables.
    ///
    /// # Returns
    /// * `Settings` - The loaded configuration settings.
    pub fn new() -> Result<Self, ConfigError> {
        let builder: ConfigBuilder<_> = Config::builder()
            .add_source(File::with_name("config/settings").required(false))
            .add_source(Environment::with_prefix("APP"));

        let settings: Config = builder.build()?;
        settings.try_deserialize::<Settings>()
    }
}
