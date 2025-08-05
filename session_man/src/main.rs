use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::CaptureScreenshotFormat,
    page::ScreenshotParams,
};
use chrono::Utc;
use config::{ApplicationConfig, PortalConfig};
use env_logger::Env;
use futures::StreamExt;
use std::io::Write;
use std::{fs::File, path::Path};

use log::{debug, error, info, warn};
mod config;
mod rpa;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().inspect_err(|e| warn!("{e}")).ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .init();
    let app_conf: ApplicationConfig = envy::from_env()
        .expect("Failed to deserialize config from environment variables");

    debug!("app config {app_conf:?}");

    let portal_conf: PortalConfig = envy::prefixed("PORTAL_")
        .from_env()
        .expect("Failed to deserialize config from environment variables");

    if !Path::new(&app_conf.screenshot_path).exists() {
        panic!("screen path {} not exist", app_conf.screenshot_path)
    }

    let mut browser_conf = BrowserConfig::builder()
        .no_sandbox()
        .new_headless_mode();

    if let Some(path) = app_conf.browser_execute_path {
        browser_conf = browser_conf.chrome_executable(Path::new(&path));
    }

    let (mut browser, mut handler) =
        Browser::launch(browser_conf.build().unwrap()).await?;

    info!("start browser");
    // Spawn the handler loop — this is REQUIRED
    tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });

    let page = browser.new_page(&portal_conf.url).await?;

    match rpa::run(portal_conf, page.clone()).await {
        Ok(cookies) => {
            let file_name =
                format!("{}/cookie-latest.json", app_conf.session_storage_path);
            let mut file = File::create(&file_name)?;
            let cookie_json = serde_json::to_string_pretty(&cookies)?;
            file.write_all(cookie_json.as_bytes())?
        }
        Err(e) => {
            error!("Result failed, trying to capture image page: {e}");
            page.screenshot(
                ScreenshotParams::builder()
                    .format(CaptureScreenshotFormat::Png)
                    .build(),
            )
            .await
            .map(|s| {
                let filename = Utc::now()
                    .format("session_man_error_%Y%m%d-%H%M%S.png")
                    .to_string();
                let full_path =
                    format!("{}/{filename}", app_conf.screenshot_path);
                match File::create(full_path.clone()) {
                    Ok(mut file) => match file.write_all(&s) {
                        Ok(_) => info!("Wrote file {full_path} success"),
                        Err(e) => error!("Save screenshot error {e}"),
                    },
                    Err(e) => {
                        error!(":/ {e}")
                    }
                }
            })
            .inspect_err(|e| error!("Err: {e}"))
            .ok();
        }
    };
    page.close().await?;
    browser.close().await?;
    Ok(())
}
