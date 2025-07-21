use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeData {
    pub provider_id: i32,
    pub provider_name: String,
    pub provider_abbreviation: String,
    pub package_type_id: i32,
    pub package_type_name: String,
    pub privilege_type_id: i32,
    pub privilege_type_name: String,
    pub privilege_display_name: String,
    pub privilege_flow_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Privilege {
    pub privilege_provider_id: i32,
    pub provider_name: String,
    pub provider_abbreviation: String,
    pub privilege_packages: Vec<PrivilegePackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivilegePackage {
    pub privilege_id: i32,
    pub package_type_id: i32,
    pub package_type_name: String,
    pub display_name: String,
    pub instruction_html: String,
    pub company_logo_url: String,
    pub legacy_provider_id: Option<i32>,
}
