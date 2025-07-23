use std::io::prelude::*;
use std::{error::Error, sync::Arc};

use futures::FutureExt;
use futures::future::OrElse;
use protocol::biz_priv::binding::BindingData;
use protocol::iam::user_identity::UserIdentity;

use crate::rpa::models::{GroupPolicyRequestBinding, RequestBinding};
use crate::{repo::add_policy::AddPolicyRepo, rpa::check_privilege::BindingPortalAutomation};

pub trait AddPolicyService {
    async fn add_new_user_policy(
        &self,
        user_id: UserIdentity,
        req: BindingData,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct AddPolicyServiceImp {
    repo: Arc<dyn AddPolicyRepo>,
    rpa: Arc<dyn BindingPortalAutomation>,
}

impl AddPolicyService for AddPolicyServiceImp {
    async fn add_new_user_policy(
        &self,
        user_id: UserIdentity,
        req: BindingData,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let binding_data = match req {
            BindingData::ThaiCitizenCard(t) => RequestBinding::Group(GroupPolicyRequestBinding {
                policy_holder_ref: t.custom_data.policy_number,
                insurred_member: t.custom_data.member_number,
            }),
            _ => return Err(Box::from("String")),
        };

        let policy = self.rpa.get_policy(binding_data).await?;
        self.repo.add_binding_tx(user_id, policy).await?;
        Ok(())
    }
}
