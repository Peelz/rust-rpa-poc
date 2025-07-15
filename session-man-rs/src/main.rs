use std::{
    env::{self},
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::var("SCREENSHOT_PATH")?;
    let browser_opt = LaunchOptions {
        window_size: Some((1025, 768)),
        ..Default::default()
    };
    // browser_opt.window_size = Some((1025, 768));

    let browser = Browser::new(browser_opt)?;
    let tab = browser.new_tab()?;

    tab.navigate_to("https://webapp.generali.co.th/eHospital/login.jsp")?;
    tab.wait_for_xpath("//input[@name='user_name']")?;

    let png_data = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;

    let file_name = SystemTime::now().duration_since(UNIX_EPOCH)?;

    std::fs::write(format!("{}/{}.png", path, file_name.as_secs()), png_data)?;

    Ok(())
}
