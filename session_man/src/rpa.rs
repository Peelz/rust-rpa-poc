use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;

use chromiumoxide::Page;
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::{cdp::browser_protocol::network::Cookie, error::CdpError};
use futures::FutureExt;
use log::{debug, error, info};
use tokio::fs;
use tokio::time::sleep;

use crate::config::PortalConfig;

#[derive(Debug)]
pub struct RpaError {
    detail: String,
}

impl From<CdpError> for RpaError {
    fn from(err: CdpError) -> Self {
        RpaError {
            detail: err.to_string(),
        }
    }
}

impl From<String> for RpaError {
    fn from(err: String) -> Self {
        RpaError { detail: err }
    }
}

impl From<&str> for RpaError {
    fn from(err: &str) -> Self {
        RpaError {
            detail: err.to_string(),
        }
    }
}

impl fmt::Display for RpaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.detail)
    }
}

impl Error for RpaError {}

pub async fn run(
    portal_conf: PortalConfig,
    page: Page, // browser: browser::Browser,
) -> Result<Vec<Cookie>, RpaError> {
    let url = format!("{}/eHospital/login.jsp", portal_conf.url);
    page.goto(&url).await?;
    debug!("Nav to login {url}");

    // let screen = page
    //     .screenshot(
    //         ScreenshotParams::builder()
    //             .format(CaptureScreenshotFormat::Png)
    //             .build(),
    //     )
    //     .await?;
    //
    // let mut file = File::create(
    //     "/Users/peelz/Workspace/biz-priv-insurance-generali/local/screenshot/login.png",
    // )
    // .map_err(|e| RpaError::from("test"))?;
    //
    // file.write_all(&screen)
    //     .map_err(|e| RpaError::from("test"))?;

    page.find_xpath("//input[@name='user_name']")
        .await
        .inspect_err(|e| error!("Colud not find user_name field"))?
        .click()
        .await?
        .type_str(portal_conf.user_name)
        .await?;

    debug!("Loggin Process");

    page.find_xpath("//input[@name='password']")
        .await
        .inspect_err(|e| error!("Colud not find password field"))?
        .click()
        .await?
        .type_str(portal_conf.user_password)
        .await?;

    page.find_xpath("//img[@src='images/login.jpg']")
        .await
        .inspect_err(|e| error!("Colud not find login button"))?
        .click()
        .await
        .inspect_err(|e| error!("Can not click login button"))?;

    page.wait_for_navigation().await?;

    sleep(Duration::from_secs(1)).await;

    let text = page
        .find_element("body > table > tbody > tr:nth-child(2) > td.indent > table > tbody > tr:nth-child(2) > td > span")
        .await
        .inspect_err(|_| error!("Can not detect landing page from selector"))?
        .inner_text()
        .await?
        .ok_or(RpaError::from(
            "Can not detect landing page from inner_text",
        ))?;

    info!("Welcome text {text}");

    if !text.contains("ยินดีต้อนรับ") {
        return Err(RpaError::from(
            "Can not verify welcome text on landing page",
        ));
    }

    Ok(page.get_cookies().await?)
}
