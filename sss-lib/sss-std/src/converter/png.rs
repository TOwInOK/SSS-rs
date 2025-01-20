use base64_light::base64_encode;
use headless_chrome::{
    protocol::cdp::Page::CaptureScreenshotFormatOption, protocol::cdp::Page::Viewport, Browser,
    LaunchOptionsBuilder,
};
use image::{Rgba, RgbaImage};
use std::{error::Error, io::Cursor, time::Duration};

pub async fn html_to_image(
    html_content: &str,
    viewport_padding: Option<f64>,
    border_radius: u32,
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

    let main_div = tab.wait_for_xpath("//div[1]/div[1]")?;

    let box_model = main_div.get_box_model()?;
    let border = &box_model.border;

    let viewport_padding = viewport_padding.unwrap_or(20.0);

    let viewport = Viewport {
        x: border.top_left.x - viewport_padding / 2.0,
        y: border.top_left.y - viewport_padding / 2.0,
        width: (border.top_right.x - border.top_left.x) + viewport_padding,
        height: (border.bottom_left.y - border.top_left.y) + viewport_padding,
        scale: 1.0,
    };

    let screenshot = tab.capture_screenshot(
        CaptureScreenshotFormatOption::Png,
        None,
        Some(viewport),
        true,
    )?;

    let img = image::load_from_memory(&screenshot)?;

    let mut img: RgbaImage = img.to_rgba8();
    let (width, height) = img.dimensions();

    let right_x = width - border_radius;
    let bottom_y = height - border_radius;

    // top left
    for x in 0..border_radius {
        for y in 0..border_radius {
            let dx = (x as i32 - border_radius as i32).abs();
            let dy = (y as i32 - border_radius as i32).abs();

            let is_within = dx * dx + dy * dy <= (border_radius * border_radius) as i32;

            if !is_within {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    // top right
    for x in right_x..width {
        for y in 0..border_radius {
            let dx = (width as i32 - x as i32 - border_radius as i32).abs();
            let dy = (y as i32 - border_radius as i32).abs();

            let is_within = dx * dx + dy * dy <= (border_radius * border_radius) as i32;

            if !is_within {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    // bottom left
    for x in 0..border_radius {
        for y in bottom_y..height {
            let dx = (x as i32 - border_radius as i32).abs();
            let dy = (height as i32 - y as i32 - border_radius as i32).abs();

            let is_within = dx * dx + dy * dy <= (border_radius * border_radius) as i32;

            if !is_within {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    // bottom right
    for x in right_x..width {
        for y in bottom_y..height {
            let dx = (width as i32 - x as i32 - border_radius as i32).abs();
            let dy = (height as i32 - y as i32 - border_radius as i32).abs();

            let is_within = dx * dx + dy * dy <= (border_radius * border_radius) as i32;

            if !is_within {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)?;

    Ok(buffer)
}
