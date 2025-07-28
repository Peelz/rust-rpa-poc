use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PortalConfig {
    // #[serde(rename = "PORTAL_URL")]
    pub url: String,

    // #[serde(rename = "PORTAL_USER_NAME")]
    pub user_name: String,

    // #[serde(rename = "PORTAL_USER_PASSWORD")]
    pub user_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub session_storage_path: String,
    pub screenshot_path: String,
}
