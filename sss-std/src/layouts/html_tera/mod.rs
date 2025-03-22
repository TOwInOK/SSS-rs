pub mod html_tera_builder;

pub use html_tera_builder::{HtmlTeraFinalize, HtmlTeraRender};
use layouts_derive::generate_layouts;
use serde::{Deserialize, Serialize};
use sss_core::Settings;
use theme::Theme;

generate_layouts!(
    "src/layouts/html_tera/templates",
    "src/layouts/html_tera/default_layout.html.tera"
);
