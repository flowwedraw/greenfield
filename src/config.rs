#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub telegram_api_hash: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        let telegram_api_hash = std::env::var("TELEGRAM_API_HASH")?;
        Ok(Self { telegram_api_hash })
    }
}
