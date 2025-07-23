use serde::{Deserialize, Serialize};

use super::BindingData;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindingRequest {
    privilege_id: u32,
    binding_data: BindingData,
}
