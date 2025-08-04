use chromiumoxide::error::CdpError;
use common::protocol::biz_priv::binding::binding_result::RejectedReason;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BindingError {
    #[error("Data Miss Matching")]
    DataMissMatch,
    #[error("Automation Failed")]
    AutomationFail(#[from] CdpError),
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

// impl From<CdpError> for AddPolicyError {
//     fn from(e: CdpError) -> Self {
//         Self {
//             cause: e.to_string(),
//         }
//     }
// }

// impl std::fmt::Display for AddPolicyError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl Error for AddPolicyError {}
