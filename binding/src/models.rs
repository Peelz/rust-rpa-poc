use serde::Serialize;
use time::OffsetDateTime;

#[derive(Debug, Serialize)]
pub struct GetPolicyResult {
    // กรมธรรม์เลขที่ :
    pub policy_ref: String,
    // วันมีผลบังคับ :
    pub active_at: OffsetDateTime,
    // วันสิ้นสุด :
    pub inactive_at: OffsetDateTime,
}

