use std::{
    env::{self},
    error::Error,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, post, web};
use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption};

#[derive(Clone)]
struct AppState {
    browser: Arc<Browser>,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let browser_opt = LaunchOptions {
        window_size: Some((1025, 768)),
        ..Default::default()
    };
    let browser = Browser::new(browser_opt)?;
    let app_state = AppState {
        browser: Arc::new(browser),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(screenshot)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    Ok(())
}

#[post("/screenshot")]
async fn screenshot(
    _req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let path = env::var("SCREENSHOT_PATH")?;
    let tab = data.browser.new_tab()?;
    tab.navigate_to("https://webapp.generali.co.th/eHospital/login.jsp")?;
    tab.wait_for_xpath("//input[@name='user_name']")?;

    let png_data = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;

    let file_name = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let full_path = format!("{}/{}.png", path, file_name.as_secs());

    println!("writing {}", full_path);

    std::fs::write(full_path, png_data)?;

    Ok(HttpResponse::Ok().finish())
}
