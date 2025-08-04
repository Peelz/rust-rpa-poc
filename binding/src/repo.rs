use std::{error::Error, sync::Arc};

use common::protocol::iam::user_identity::PartialUserIdentity;
use futures::future::BoxFuture;
use sqlx::{PgPool, types::Json};

use crate::models::GetPolicyResult;

pub trait AddPolicyRepo {
    fn add_binding_tx(
        &self,
        binding_id: i32,
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
        binding_id: i32,
        user_iden: PartialUserIdentity,
        policy: Option<GetPolicyResult>,
    ) -> BoxFuture<'_, Result<(), Box<dyn Error + Send + Sync>>> {
        Box::pin(async move {
            log::info!("Repo ex {policy:?}");
            sqlx::query("SELECT insert_policy ($1::int, $2::int, $3::int, $4)")
                .bind(binding_id)
                .bind(user_iden.user_profile_id as i64)
                .bind(user_iden.user_account_id as i64)
                .bind(policy.map(Json))
                .execute(&*self.pg_pool)
                .await?;
            Ok(())
        })
    }
}
