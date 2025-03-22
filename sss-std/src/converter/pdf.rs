use std::{error::Error, time::Duration};

use base64_light::base64_encode;
use headless_chrome::{Browser, LaunchOptionsBuilder, types::PrintToPdfOptions};

pub async fn html_to_pdf(
    html_content: &str,
    print_options: Option<PrintToPdfOptions>,
) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let launch_options = LaunchOptionsBuilder::default()
        .ignore_certificate_errors(true)
        .build()?;

    let browser = Browser::new(launch_options)?;

    let html_base64 = base64_encode(html_content);
    let data_url = format!(r#"data:text/html;base64,{}"#, &html_base64);

    let tab = browser.new_tab()?;

    tab.set_default_timeout(Duration::from_secs(3))
        .navigate_to(&data_url)?;

    tab.wait_for_element("body")?;

    // Wait font loading
    tab.evaluate(
        r#"
            new Promise((resolve) => {
                if (document.fonts && document.fonts.ready) {
                    document.fonts.ready.then(() => {
                        setTimeout(resolve, 100);
                    });
                } else {
                    setTimeout(resolve, 500);
                }
            })
        "#,
        true,
    )?;

    tab.wait_for_xpath("//div[1]/div[1]")?;

    let pdf_options = print_options.unwrap_or(PrintToPdfOptions {
        print_background: Some(true),
        scale: Some(1.0),
        paper_width: Some(8.27),  // A4
        paper_height: Some(11.7), // A4
        prefer_css_page_size: Some(true),
        ..Default::default()
    });

    let pdf_data = tab.print_to_pdf(Some(pdf_options))?;
    Ok(pdf_data)
}
