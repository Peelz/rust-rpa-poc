use serde::{Deserialize, Serialize};

use crate::{binding::BindingData, category::Category};

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckEliRequest {
    pub account_id: i32,
    pub profile_id: i32,
    pub tenant_id: i32,
    pub binding_info: BindingInfo,
    pub claim_info: ClaimInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindingInfo {
    pub binding_id: i64,
    pub privilege_ref_id: Option<String>,
    pub accepted_consent_ref: Option<String>,
    pub binding_data: BindingData,
    pub provider_id: i32,
    pub provider_name: String,
    pub provider_abbreviation: String,
    pub package_type_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimInfo {
    pub payment_ref: String,
    pub booking_id: String,
    pub booking_date_time: i64,
    pub pending_claim_amount: f64, // or use rust_decimal::Decimal
    pub request_claim_amount: f64,
    pub total_claim_amount: f64,
    pub category: Category,
}
