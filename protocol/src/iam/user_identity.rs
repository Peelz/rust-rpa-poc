use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentity {
    pub account_id: u64,
    pub account_type: u64,
    pub user_profile_id: u64,
    pub user_main_profile_id: u64,
    pub tenant_id: u64,
    pub oidc_user_id: Option<String>,
    pub legacy_data: Option<LegacyData>,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyData {
    user_id: String,
    uid: u64,
    client_id: String,
    client_int_id: u64,
    scopes: String,
    role_code: String,
}
