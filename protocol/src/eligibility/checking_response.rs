use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum CheckEliResponse {
    CoverageResult(CoverageResult),
    PreconditionFailed(PreconditionFailed),
    ServiceUnavailable(ServiceUnavailable),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoverageResult {
    pub coverage_amount: f64, // or Decimal
    pub quota_amount: f64,
    pub coverage_type: CoverageType,
    pub reference_id: Option<String>,
    pub custom_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CoverageType {
    FullCoverage,
    Copay,
    Coinsurance,
    Others,
    Undefined,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreconditionFailed {
    pub fail_type: FailType,
    pub display_message: String,
    pub response_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FailType {
    PrivilegeExpired,
    PrivilegeDisabled,
    PrivilegeNotFound,
    PrivilegePending,
    ExceedQuotaPerDay,
    Others,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceUnavailable {
    pub response_code: i32,
    pub response_message: String,
}

