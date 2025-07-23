use std::error::Error;

use futures::future::BoxFuture;
use time::OffsetDateTime;

use super::models::RequestBinding;

#[derive(Debug)]
pub struct Policy {
    // กรมธรรม์เลขที่ :
    policy_ref: String,
    // วันมีผลบังคับ :
    active_at: OffsetDateTime,
    // วันสิ้นสุด :
    inactive_at: OffsetDateTime,
}

pub trait BindingPortalAutomation {
    fn get_policy(
        &self,
        req: RequestBinding,
    ) -> BoxFuture<Result<Option<Policy>, Box<dyn Error + Send + Sync>>>;
}


