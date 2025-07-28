use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::{network::Cookie, page::CaptureScreenshotFormat},
    page::ScreenshotParams,
};
use chrono::Utc;
use config::{ApplicationConfig, PortalConfig};
use env_logger::Env;
use futures::{StreamExt, future::ok};
use std::io::Write;
use std::{fs::File, path::Path};

use log::{error, info};
mod config;
mod rpa;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_conf: ApplicationConfig = envy::from_env()
        .expect("Failed to deserialize config from environment variables");

    let portal_conf: PortalConfig = envy::prefixed("PORTAL_")
        .from_env()
        .expect("Failed to deserialize config from environment variables");

    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .init();

    if !Path::new(&app_conf.screenshot_path).exists() {
        panic!("screen path {} not exist", app_conf.screenshot_path)
    }

    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            // .with_head()
            .new_headless_mode()
            .window_size(1024, 728)
            .build()?,
    )
    .await?;

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
    Ok(())
}
