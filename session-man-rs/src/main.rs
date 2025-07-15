use std::error::Error;

use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption};

fn main() -> Result<(), Box<dyn Error>> {
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

    std::fs::write("screenshot.png", png_data)?;

    Ok(())
}
