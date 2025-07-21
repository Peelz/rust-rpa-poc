use std::error::Error;

use config::{Config, Environment};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ApplicationConfig {
    pub portal_url: String,
    pub http_server_port: u16,
}

pub fn load_env() -> Result<ApplicationConfig, Box<dyn Error>> {
    dotenv().ok(); // Load from .env if present

    let config = Config::builder()
        .add_source(Environment::default().separator("__")) // ENV overrides
        .build()?;

    config
        .try_deserialize::<ApplicationConfig>()
        .map_err(Into::into)
}
