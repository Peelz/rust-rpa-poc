use futures::StreamExt;
use std::{
    env::{self},
    error::Error,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, post, web};
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::CaptureScreenshotFormat,
};
use log::info;

#[derive(Clone)]
struct AppState {
    browser: Arc<Browser>,
    screenshot_dir: String,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = env::var("SCREENSHOT_PATH")?;

    // env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    //
    // let download_path = Path::new("./download");
    // tokio::fs::create_dir_all(&download_path).await?;
    // let fetcher = BrowserFetcher::new(
    //     BrowserFetcherOptions::builder()
    //         .with_path(download_path)
    //         .build()?,
    // );
    // let _info = fetcher.fetch().await?;

    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .new_headless_mode()
            .window_size(1024, 728)
            .build()?,
    )
    .await?;

    // Spawn the handler loop — this is REQUIRED
    tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });

    let app_state = AppState {
        browser: Arc::new(browser),
        screenshot_dir: path,
    };

    let http_server_config = ("0.0.0.0", 8080);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(screenshot)
    })
    .bind(http_server_config)?
    .run()
    .await?;

    Ok(())
}

#[post("/screenshot")]
async fn screenshot(
    _req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Box<dyn Error>> {
    info!("new comming request");
    let png_data = data
        .browser
        .new_page("https://webapp.generali.co.th/eHospital/login.jsp")
        .await?
        .find_xpath("//input[@name='user_name']")
        .await
        .unwrap()
        .click()
        .await
        .unwrap()
        .type_str("Test")
        .await?
        .screenshot(CaptureScreenshotFormat::Png)
        .await?;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let filename = format!("{}/{}.png", data.screenshot_dir, timestamp.as_secs());
    info!("Saved image {filename}");
    tokio::fs::write(filename, png_data).await?;
    Ok(HttpResponse::Ok().body(format!("Saved screenshot as {}", timestamp.as_secs())))
}
