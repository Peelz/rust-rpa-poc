use std::error::Error;

use common::gcp::pubsub::PublisherConfig;
use config::{Config, Environment};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PostgresConfig {
    user_name: String,
    password: String,
    host: String,
    port: Option<u16>,
    db_name: String,
}

impl PostgresConfig {
    pub fn to_url(&self) -> String {
        format!(
            "posrgres://{}:{}@{}:{}/{}",
            self.user_name,
            self.password,
            self.host,
            self.port.unwrap_or(5432),
            self.db_name
        )
    }
}


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ApplicationConfig {
    pub portal_url: String,
    pub session_path: String,
    pub http_server_port: u16,
    pub postgres: PostgresConfig,
    pub gcp_project_id: String,
    pub gcp_binding_result_topic: String,
    pub gcp_pubsub_emulator: bool,
}

pub fn load_env() -> Result<ApplicationConfig, Box<dyn Error>> {
    dotenv().ok(); // Load from .env if present

    let config = Config::builder()
        .add_source(Environment::default().separator("_")) // ENV overrides
        .build()?;

    config
        .try_deserialize::<ApplicationConfig>()
        .map_err(Into::into)
}
