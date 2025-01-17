pub mod html_tera_builder;
pub mod templates;

use html_tera_builder::HtmlTeraRender;
use render::{layout::Finalize, theme::Theme};
use serde::{Deserialize, Serialize};
use sss_core::Settings;
use templates::{UMBRELLA_TERA_CARD_TEMPLATE, UMBRELLA_TERA_TEMPLATE};

#[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum)]
pub enum Layouts {
    #[default]
    Umbrella,
}
impl Layouts {
    pub fn to_layout<'a>(
        &self,
        settings: &'a Settings,
        theme: &'static Theme,
    ) -> Box<impl Finalize + 'a> {
        match self {
            Layouts::Umbrella => Box::new(HtmlTeraRender::new(
                settings,
                theme,
                UMBRELLA_TERA_CARD_TEMPLATE,
                UMBRELLA_TERA_TEMPLATE,
            )),
        }
    }
}

use std::fmt;

impl fmt::Display for Layouts {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Layouts::Umbrella => write!(f, "umbrella"),
        }
    }
}
