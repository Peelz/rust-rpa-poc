use chromiumoxide::error::CdpError;
use common::protocol::biz_priv::binding::binding_result::RejectedReason;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BindingError {
    #[error("Data Miss Matching")]
    DataMissMatch,
    #[error("Automation Fail {0}")]
    AutomationFailure(#[from] AutomationError),
    #[error("Internal Server Error")]
    InternalServerError(#[from] anyhow::Error),
    #[error("InvalidDataHandle")]
    InvalidDataHandle,
}

#[derive(Debug)]
pub struct AddPolicyError {
    cause: String,
}

impl From<RejectedReason> for AddPolicyError {
    fn from(value: RejectedReason) -> Self {
        Self {
            cause: value.to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum AutomationError {
    #[error("CdpError {0}")]
    BrowserProtocolError(#[from] CdpError),
    #[error("Element {target_name} not found, with {source}")]
    ElementNotFoundWith {
        target_name: String,
        source: CdpError,
    },
    #[error("Element {target_name} not found")]
    ElementNotFound { target_name: String },
    #[error("Date parse fail {field}")]
    DateParserFail { field: String },
}
