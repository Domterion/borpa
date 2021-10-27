use std::{fs, net::SocketAddr, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub metrics: MetricsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    pub token: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsConfig {
    pub address: SocketAddr,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        config
    }
}
