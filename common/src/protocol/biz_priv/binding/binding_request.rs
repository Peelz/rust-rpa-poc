use serde::{Deserialize, Serialize};

use super::{binding_data::BindingData, privilege::PrivilegeData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingRequestEvent {
    pub privilege_id: u64,
    pub binding_id: u64,
    pub account_id: u64,
    pub profile_id: u64,
    pub accepted_consent_ref: String,
    pub binding_data: BindingData,
    pub encrypted_binding_data: String,
    pub encryption_key_urn: String,
    pub privilege_data: PrivilegeData,
    pub session_id: String,
}
