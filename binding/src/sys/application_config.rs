use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct PostgresConfig {
    user_name: String,
    password: String,
    host: String,
    port: Option<u16>,
    db_name: String,
}

impl PostgresConfig {
    pub fn to_url(&self) -> String {
        let mut url = Url::parse("postgres://localhost").unwrap();
        url.set_username(&self.user_name).unwrap();
        url.set_password(Some(&self.password)).unwrap();
        url.set_host(Some(&self.host)).unwrap();
        url.set_port(self.port).unwrap_or(());
        url.set_path(&self.db_name);
        url.to_string()
    }
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationConfig {
    pub portal_url: String,
    pub session_path: String,
    pub screenshot_path: String,
    pub http_server_port: u16,
    pub gcp_project_id: String,
    pub gcp_binding_result_topic: String,
    pub pubsub_emulator_host: Option<String>,
    #[serde(default="default_false")]
    pub headless_mode: bool
}
