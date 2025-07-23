use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__type")]
pub enum BindingData {
    ThaiCitizenCard(ThaiCitizenCard),
    Passport(Passport),
    EmployeeInfo(EmployeeInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomData {
    pub member_number: String,
    pub policy_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThaiCitizenCard {
    pub citizen_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    pub custom_data: CustomData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Passport {
    pub passport_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeInfo {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, String>>,
}
