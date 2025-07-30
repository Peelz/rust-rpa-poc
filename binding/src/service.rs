use core::fmt;
use std::error::Error;
use std::sync::Arc;

use crate::error::BindingError;
use crate::repo::AddPolicyRepo;
use crate::rpa::{BindingPortalAutomation, GroupPolicyRequestBinding};
use common::protocol::biz_priv::binding::binding_data::BindingData;
use common::protocol::biz_priv::binding::binding_request::{
    self, BindingRequestEvent,
};
use common::protocol::biz_priv::binding::binding_result::RejectedReason;

pub struct BindingDataExt(pub BindingData);

impl BindingDataExt {
    fn into_request_binding(
        self,
    ) -> Result<GroupPolicyRequestBinding, RejectedReason> {
        match self.0 {
            BindingData::ThaiCitizenCard(t) => Ok(GroupPolicyRequestBinding {
                policy_holder_ref: t.custom_data.policy_number,
                insurred_member: t.custom_data.member_number,
            }),
            _ => Err(RejectedReason::PreconditionFailed),
        }
    }
}

pub struct AddPolicyServiceImp {
    repo: Box<dyn AddPolicyRepo>,
    automation: Box<dyn BindingPortalAutomation>,
    result_publisher: google_cloud_pubsub::publisher::Publisher,
}

impl AddPolicyServiceImp {
    pub fn new(
        repo: Box<dyn AddPolicyRepo>,
        automation: Box<dyn BindingPortalAutomation>,
        result_publisher: google_cloud_pubsub::publisher::Publisher,
    ) -> Self {
        Self {
            repo,
            automation,
            result_publisher,
        }
    }

    pub async fn add_policy(
        &self,
        event: BindingRequestEvent,
    ) -> Result<(), Box<dyn Error>> {
        let binding_req = BindingDataExt(event.binding_data)
            .into_request_binding()
            .map_err(|e| Box::new(BindingError::DataMissMatch))?;

        let policy = self.automation.get_policy(binding_req).await?;

        Ok(())
    }
}
