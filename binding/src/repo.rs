use std::{error::Error, sync::Arc};

use common::protocol::iam::user_identity::PartialUserIdentity;
use futures::future::BoxFuture;
use sqlx::{PgPool, types::Json};

use crate::models::GetPolicyResult;

pub struct UserIdentity {
    pub account_id: i32,
    pub profile_id: i32,
}

pub trait AddPolicyRepo {
    fn add_binding_tx(
        &self,
        user_iden: PartialUserIdentity,
        policy: Option<GetPolicyResult>,
    ) -> BoxFuture<Result<(), Box<dyn Error + Send + Sync>>>;
}

pub struct AddPolicyRepoImp {
    pg_pool: Arc<PgPool>,
}

impl AddPolicyRepoImp {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool }
    }
}

impl AddPolicyRepo for AddPolicyRepoImp {
    fn add_binding_tx(
        &self,
        user_iden: PartialUserIdentity,
        policy: Option<GetPolicyResult>,
    ) -> BoxFuture<'_, Result<(), Box<dyn Error + Send + Sync>>> {
        Box::pin(async move {
            sqlx::query("SELECT insert_policy ($1, $2)")
                .bind(user_iden.user_account_id as i64)
                .bind(policy.map(Json))
                .execute(&*self.pg_pool)
                .await?;
            Ok(())
        })
    }
}
