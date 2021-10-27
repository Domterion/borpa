use std::{fs, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    pub token: String,
    pub version: String,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        config
    }
}
