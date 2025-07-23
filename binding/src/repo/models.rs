use time::OffsetDateTime;


#[derive(Debug)]
pub struct BindingTx {
    user_account_id: u32,
    user_profile_id: u32,
    evidence: String,
    created_at: OffsetDateTime,
}
