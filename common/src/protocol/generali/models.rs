use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeneftRecordV1 {
    // รายละเอียดผลประโยชน์, ex: OPD Lump sum
    pub benefit_name: String,

    // ผลประโยชน์
    pub benefit_total_amount: bigdecimal::BigDecimal,

    // ผลประโยชน์ใช้ไปแล้ว
    pub benefit_usage_amount: bigdecimal::BigDecimal,

    // ผลประโยชน์คงเหลือ
    pub benefit_remaining_amount: bigdecimal::BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__type", rename_all = "camelCase")]
pub enum GeneraliPolicyInfo {
    V1 {
        // กรมธรรม์เลขที่ :
        policy_ref: String,

        // วันมีผลบังคับ :
        #[serde(with = "time::serde::rfc3339")]
        active_at: OffsetDateTime,
        // วันสิ้นสุด :
        #[serde(with = "time::serde::rfc3339")]
        inactive_at: OffsetDateTime,

        benefit: BeneftRecordV1,
    },
}
