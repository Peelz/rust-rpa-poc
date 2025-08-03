use chromiumoxide::{
    Browser, BrowserConfig, Element, Page, browser,
    cdp::browser_protocol::{
        network::{Cookie, CookieParam, SetCookieParams},
        storage::{SetCookiesParams, SetCookiesParamsBuilder},
    },
    error::CdpError,
};
use common::utils::parse_buddhist_date;
use futures::{StreamExt, future::BoxFuture};
use log::info;

use crate::{
    error::{self, BindingError},
    models::GetPolicyResult,
    service,
};

#[derive(Debug, Clone)]
pub struct GroupPolicyRequestBinding {
    pub policy_holder_ref: String,
    pub insurred_member: String,
}

pub trait BindingPortalAutomation {
    fn get_policy(
        &self,
        req: GroupPolicyRequestBinding,
    ) -> BoxFuture<Result<Option<GetPolicyResult>, BindingError>>;
}

pub struct BindingPortalAutomationImp {
    browser: Browser,
    base_portal_url: String,
    // cookies: Vec<CookieParam>,
}

impl BindingPortalAutomationImp {
    pub(crate) async fn new(
        cookies: Vec<CookieParam>,
        base_portal_url: String,
    ) -> Self {
        let (browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .no_sandbox()
                .with_head()
                // .new_headless_mode()
                .window_size(1024, 728)
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

        info!("start browser");

        // Spawn the handler loop — this is REQUIRED
        tokio::task::spawn(async move {
            loop {
                let _ = handler.next().await.unwrap();
            }
        });
        let page = browser.new_page(&base_portal_url).await.unwrap();
        page.execute(SetCookiesParams::new(cookies))
            .await
            .inspect_err(|e| log::error!("set cookies failed {e}"))
            .unwrap();

        page.close().await;

        Self {
            browser,
            base_portal_url,
        }
    }

    async fn exec_search_policy(
        &self,
        page: &Page,
        policy_input: Vec<&str>,
        member_id: Vec<&str>,
    ) -> Result<(), CdpError> {
        page.find_xpath("//input[@name='tpolicyType']").await?;
        page.find_xpath("//input[@name='gl_polnum1']")
            .await?
            .type_str(policy_input[0])
            .await?;
        page.find_xpath("//input[@name='gl_polnum2']")
            .await?
            .type_str(policy_input[1])
            .await?;
        page.find_xpath("//input[@name='cert_no']")
            .await?
            .type_str(member_id[0])
            .await?;
        page.find_xpath("//input[@name='cert_prefix']")
            .await?
            .type_str(member_id[1])
            .await?;
        page.find_xpath("//input[@name='cert_prefix']")
            .await?
            .type_str(member_id[1])
            .await?;
        page.find_xpath("//img[@src='images/search.jpg']")
            .await?
            .click()
            .await?;

        page.wait_for_navigation().await?;
        Ok(())
    }

    async fn exec_get_policy(
        &self,
        page: &Page,
    ) -> Result<GetPolicyResult, Box<dyn std::error::Error>> {
        let policy_ref = page
            .find_xpath("/html/body/table/tbody/tr[2]/td[2]/table/tbody/tr[3]/td[2]/form/table/tbody/tr[4]/td[2]/input")
            .await?
            .attribute("value")
            .await?
            .unwrap_or_default();

        let active_at = page
            .find_xpath("/html/body/table/tbody/tr[2]/td[2]/table/tbody/tr[3]/td[2]/form/table/tbody/tr[6]/td[2]/input") 
            .await?
            .attribute("value")
            .await?
            .unwrap_or_default();

        let inactive_at = page
            .find_xpath("/html/body/table/tbody/tr[2]/td[2]/table/tbody/tr[3]/td[2]/form/table/tbody/tr[7]/td[4]/input") 
            .await?
            .attribute("value")
            .await?
            .unwrap_or_default();

        let result = GetPolicyResult {
            policy_ref,
            active_at: parse_buddhist_date(&active_at.to_string())?,
            inactive_at: parse_buddhist_date(&inactive_at.to_string())?,
        };

        Ok(result)
    }
}

impl BindingPortalAutomation for BindingPortalAutomationImp {
    fn get_policy(
        &self,
        req: GroupPolicyRequestBinding,
    ) -> BoxFuture<Result<Option<GetPolicyResult>, BindingError>> {
        let url =
            format!("{}/eHospital/EnquiryPolicy.gt", self.base_portal_url);
        Box::pin(async move {
            let page = self.browser.new_page(url).await?;
            let policy_input: Vec<&str> =
                req.policy_holder_ref.split("-").collect();
            let member_id: Vec<&str> = req.insurred_member.split("-").collect();

            self.exec_search_policy(&page, policy_input, member_id)
                .await?;

            let result = self.exec_get_policy(&page).await.ok();

            if let Err(e) = page.close().await {
                log::warn!("Failed to close page {:?}", e)
            }
            // let result = result.ok();
            Ok(result)
        })
    }
}
