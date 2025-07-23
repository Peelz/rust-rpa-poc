use std::{error::Error, pin::Pin, sync::Arc};

use futures::future::BoxFuture;
use protocol::iam::user_identity::{self, UserIdentity};
use sqlx::{Executor, PgPool};

use crate::rpa::check_privilege::Policy;

pub trait AddPolicyRepo {
    fn add_binding_tx(
        &self,
        user_identity: UserIdentity,
        policy: Option<Policy>,
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
        user_identity: UserIdentity,
        policy: Option<Policy>,
    ) -> BoxFuture<Result<(), Box<dyn Error + Send + Sync>>> {
        Box::pin(async move {
            sqlx::query("SELECT insert_policy ($1)")
                .bind(user_identity.account_id as i64)
                .fetch(&*self.pg_pool);
            Ok(())
        })
    }
}
