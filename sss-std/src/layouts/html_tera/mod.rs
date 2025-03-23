pub mod html_meta;
pub mod html_tera_builder;
use html_layouts_derive::generate_layouts;
pub use html_tera_builder::{HtmlTeraFinalize, HtmlTeraRender};
use serde::{Deserialize, Serialize};
use sss_core::Settings;
use theme::Theme;

generate_layouts!(
    "src/layouts/html_tera/templates",
    "src/layouts/html_tera/default_layout.html.tera"
);
