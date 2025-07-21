use crate::binding::binding_data::BindingData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindingRequest {
    privilege_id: u32,
    binding_data: BindingData,
}
