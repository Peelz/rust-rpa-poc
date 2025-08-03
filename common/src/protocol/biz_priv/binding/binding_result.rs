use serde::{Deserialize, Serialize};

use super::privilege::PrivilegeData;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum BindingResult {
    #[serde(rename = "CompletedBinding")]
    #[serde(rename_all = "camelCase")]
    CompletedBinding {
        binding_id: i32,
        privilege_id: i32,
        privilege_ref_id: Option<String>,
        account_id: i32,
        profile_id: i32,
        started_at: Option<i64>,
        expired_at: Option<i64>,
        accepted_consent_ref: String,
        privilege_data: PrivilegeData,
        legacy_company_id: Option<i32>,
    },

    #[serde(rename = "RejectedBinding")]
    #[serde(rename_all = "camelCase")]
    RejectedBinding {
        binding_id: i32,
        reason: RejectedReason,
        detail: String,
        privilege_data: PrivilegeData,
    },

    #[serde(rename = "FailedBinding")]
    #[serde(rename_all = "camelCase")]
    FailedBinding {
        binding_id: i32,
        reason: FailedReason,
        detail: String,
        privilege_data: PrivilegeData,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RejectedReason {
    #[serde(rename = "NOT_FOUND_BENEFIT")]
    NotFoundBenefit,
    #[serde(rename = "PRECONDITION_FAILED")]
    PreconditionFailed,
    #[serde(rename = "OTHERS")]
    Others,
}

impl std::fmt::Display for RejectedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RejectedReason::NotFoundBenefit => "NOT_FOUND_BENEFIT",
            RejectedReason::PreconditionFailed => "PRECONDITION_FAILED",
            RejectedReason::Others => "OTHERS",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailedReason {
    #[serde(rename = "PARTNER_SERVER_ERROR")]
    PartnerServerError,
    #[serde(rename = "MORDEE_SERVER_ERROR")]
    MordeeServerError,
    #[serde(rename = "OTHERS")]
    Others,
}

impl TryInto<Vec<u8>> for BindingResult {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(&self)
    }
}
