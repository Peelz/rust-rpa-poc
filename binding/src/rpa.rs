use std::{fs::File, io::Write, time::Duration};

use chromiumoxide::{
    Browser, BrowserConfig, Page,
    cdp::browser_protocol::{
        network::CookieParam, page::CaptureScreenshotFormat,
        storage::SetCookiesParams,
    },
    page::ScreenshotParams,
};
use common::utils::parse_buddhist_date;
use futures::{StreamExt, future::BoxFuture};
use log::info;
use tokio::time::sleep;

use crate::{error::BindingError, models::GetPolicyResult};

#[derive(Debug, Clone)]
pub struct GroupPolicyRequestBinding {
    pub policy_holder_ref: String,
    pub insurred_member: String,
}

pub trait BindingPortalAutomation {
    fn get_policy(
        &self,
        binding_id: i32,
        req: GroupPolicyRequestBinding,
    ) -> BoxFuture<Result<Option<GetPolicyResult>, BindingError>>;
}

pub struct BindingPortalAutomationImp {
    browser: Browser,
    base_portal_url: String,
    screenshot_path: String,
}

impl BindingPortalAutomationImp {
    pub(crate) async fn new(
        cookies: Vec<CookieParam>,
        base_portal_url: String,
        screenshot_path: String,
    ) -> Self {
        let (browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .no_sandbox()
                // .with_head()
                .new_headless_mode()
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

        // page.close().await;

        Self {
            browser,
            base_portal_url,
            screenshot_path,
        }
    }

    async fn exec_search_policy(
        &self,
        page: &Page,
        policy_input: Vec<&str>,
        member_id: Vec<&str>,
    ) -> Result<(), BindingError> {
        log::info!("Searching policy");
        page.find_xpath("//input[@type='radio' and @value='GL']")
            .await?
            .click()
            .await?;
        page.find_xpath("//input[@name='gl_polnum1']")
            .await?
            .click()
            .await?
            .type_str(policy_input[0])
            .await?;
        page.find_xpath("//input[@name='gl_polnum2']")
            .await?
            .click()
            .await?
            .type_str(policy_input[1])
            .await?;
        page.find_xpath("//input[@name='cert_no']")
            .await?
            .click()
            .await?
            .type_str(member_id[0])
            .await?;
        page.find_xpath("//input[@name='cert_prefix']")
            .await?
            .click()
            .await?
            .type_str(member_id[1])
            .await?;
        page.find_xpath("//img[@src='images/search.jpg']")
            .await?
            .click()
            .await?;

        sleep(Duration::from_millis(500)).await;
        page.wait_for_navigation().await?;
        Ok(())
    }

    async fn exec_get_policy(
        &self,
        page: &Page,
    ) -> Result<GetPolicyResult, BindingError> {
        let policy_ref = page
            .find_element("body > table > tbody > tr:nth-child(2) > td.indent > table > tbody > tr:nth-child(3) > td:nth-child(2) > form > table > tbody > tr:nth-child(4) > td:nth-child(2) > input[type=text]")
            .await
            .inspect_err(|e| log::error!("policy_ref element not found {e}"))?
            .attribute("value")
            .await?
            .ok_or_else(|| anyhow::anyhow!("Missing policy_ref"))
            .inspect_err(|e| log::error!("{e}"))?;

        let active_at = page
            .find_xpath("/html/body/table/tbody/tr[2]/td[2]/table/tbody/tr[3]/td[2]/form/table/tbody/tr[7]/td[4]/input") 
            .await
            .inspect_err(|_| log::error!("active_at element not found"))?
            .attribute("value")
            .await?
            .ok_or_else(|| anyhow::anyhow!("Missing active_at"))
            .inspect_err(|e| log::error!("{e}"))?;

        let inactive_at = page
            .find_xpath("/html/body/table/tbody/tr[2]/td[2]/table/tbody/tr[3]/td[2]/form/table/tbody/tr[7]/td[4]/input") 
            .await
            .inspect_err(|_| log::error!("inactive_at element not found"))?
            .attribute("value")
            .await?
            .ok_or_else(|| anyhow::anyhow!("Missing inactive_at"))
            .inspect_err(|e| log::error!("{e}"))?;

        log::info!("Parsing date time");

        let result = GetPolicyResult {
            policy_ref,
            active_at: parse_buddhist_date(&active_at.to_string())
                .map_err(|_| BindingError::InvalidDataHandle)
                .inspect_err(|_| {
                    log::error!("Invalid active_at {active_at}")
                })?,
            inactive_at: parse_buddhist_date(&inactive_at.to_string())
                .map_err(|_| BindingError::InvalidDataHandle)
                .inspect_err(|_| {
                    log::error!("Invalid inactive_at {inactive_at}")
                })?,
        };

        Ok(result)
    }
}

async fn on_rpa_fail(page: Page, binding_id: i32, screenshot_path: &str) -> () {
    page.screenshot(
        ScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .build(),
    )
    .await
    .map(|s| {
        let filename = format!("rpa_fail_{binding_id}.png");
        let full_path = format!("{screenshot_path}/{filename}");
        match File::create(full_path.clone()) {
            Ok(mut file) => match file.write_all(&s) {
                Ok(_) => info!("on_rpa_fail wrote file {full_path} success"),
                Err(e) => log::error!("Save screenshot error {e}"),
            },
            Err(e) => {
                log::error!(":/ {e}")
            }
        }
    })
    .inspect_err(|e| log::error!("Err: {e}"))
    .ok();
}

impl BindingPortalAutomation for BindingPortalAutomationImp {
    fn get_policy(
        &self,
        binding_id: i32,
        req: GroupPolicyRequestBinding,
    ) -> BoxFuture<Result<Option<GetPolicyResult>, BindingError>> {
        let url =
            format!("{}/eHospital/EnquiryPolicy.gt", self.base_portal_url);

        Box::pin(async move {
            let page = self.browser.new_page(url).await?;

            let policy_input: Vec<&str> =
                req.policy_holder_ref.split("-").collect();

            let member_id: Vec<&str> = req.insurred_member.split("-").collect();

            match self
                .exec_search_policy(&page, policy_input, member_id)
                .await
            {
                Ok(_) => (),
                Err(err) => {
                    log::error!("Searching policy fail");
                    on_rpa_fail(page, binding_id, &self.screenshot_path).await;
                    return Err(err);
                }
            }

            match self.exec_get_policy(&page).await {
                Ok(policy) => Ok(Some(policy)),
                Err(err) => {
                    log::error!("Getting policy info fail");
                    on_rpa_fail(page, binding_id, &self.screenshot_path).await;
                    Err(err)
                }
            }
        })
    }
}
