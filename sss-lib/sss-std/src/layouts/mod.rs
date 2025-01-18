pub mod html_tera_builder;

use html_tera_builder::HtmlTeraRender;
use layouts_derive::generate_layouts;
use render::{layout::Finalize, theme::Theme};
use serde::{Deserialize, Serialize};
use sss_core::Settings;
pub const DEFAULT_TERA_LAYOUT: &str = include_str!("default_layout.html.tera");

generate_layouts!("src/layouts/templates");
