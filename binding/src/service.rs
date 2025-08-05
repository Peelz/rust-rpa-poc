use std::error::Error;
use std::sync::Arc;

use crate::error::BindingError;
use crate::repo::AddPolicyRepo;
use crate::rpa::{GetPolicyAutomation, GroupPolicyRequestBinding};
use common::protocol::biz_priv::binding::binding_data::BindingData;
use common::protocol::biz_priv::binding::binding_request::BindingRequestEvent;
use common::protocol::biz_priv::binding::binding_result::{
    BindingResult, RejectedReason,
};
use common::protocol::generali::models::GeneraliPolicyInfo;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::publisher::Publisher;

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
    repo: Arc<dyn AddPolicyRepo + Send + Sync>,
    automation: Arc<dyn GetPolicyAutomation + Send + Sync>,
    result_publisher: Publisher,
}

impl AddPolicyServiceImp {
    pub fn new(
        repo: Arc<dyn AddPolicyRepo + Send + Sync>,
        automation: Arc<dyn GetPolicyAutomation + Send + Sync>,
        result_publisher: Publisher,
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
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let binding_req = BindingDataExt(event.binding_data)
            .into_request_binding()
            .map_err(|e| Box::new(BindingError::DataMissMatch))?;

        let policy = self
            .automation
            .get_policy(event.binding_id, binding_req)
            .await
            .inspect_err(|e| log::error!("automation error {e}"))?;

        let user_iden =
            common::protocol::iam::user_identity::PartialUserIdentity {
                user_account_id: event.account_id,
                user_profile_id: event.profile_id,
            };

        self.repo
            .add_binding_tx(event.binding_id, user_iden, policy.clone())
            .await?;

        let binding_result_msg = match policy {
            Some(GeneraliPolicyInfo::V1 {
                policy_ref,
                active_at,
                inactive_at,
                benefit,
            }) => {
                BindingResult::CompletedBinding {
                            binding_id: event.binding_id,
                            privilege_id: event.privilege_id,
                            privilege_ref_id: Some(policy_ref),
                            account_id: event.account_id,
                            profile_id: event.profile_id,
                            started_at: Some(active_at.to_utc().unix_timestamp()),
                            expired_at: Some(inactive_at.to_utc().unix_timestamp()),
                            accepted_consent_ref: "".to_string(),
                            privilege_data: event.privilege_data,
                            legacy_company_id: None,
                        }
            },
            None => BindingResult::RejectedBinding {
                binding_id: event.binding_id,
                reason: RejectedReason::NotFoundBenefit,
                detail: "".to_string(),
                privilege_data: event.privilege_data,
            },
        };

        let awaiter = self
            .result_publisher
            .publish(PubsubMessage {
                data: binding_result_msg.try_into()?,
                ..Default::default()
            })
            .await;

        match awaiter.get().await {
            Ok(_) => log::info!("Publish binding result success"),
            Err(e) => log::error!("Publish binding result fail {e:?}"),
        }
        Ok(())
    }
}
