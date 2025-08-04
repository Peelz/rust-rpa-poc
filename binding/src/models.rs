use serde::Serialize;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize)]
pub struct GetPolicyResult {
    // กรมธรรม์เลขที่ :
    pub policy_ref: String,
    // วันมีผลบังคับ :
    #[serde(with = "time::serde::rfc3339")]
    pub active_at: OffsetDateTime,
    // วันสิ้นสุด :
    #[serde(with = "time::serde::rfc3339")]
    pub inactive_at: OffsetDateTime,
}
