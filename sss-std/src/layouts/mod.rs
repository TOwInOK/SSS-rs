use render::{
    layout::{Finalize, Layout},
    theme::Theme,
};
use serde::{Deserialize, Serialize};
use sss_core::Settings;
use umbrella::UmbrellaHtmlTeraRender;

pub mod umbrella;

#[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum)]
pub enum Layouts {
    #[default]
    Umbrella,
}
#[allow(clippy::type_complexity)]
impl Layouts {
    pub fn to_layout<'a, 'b>(
        &self,
        settings: &'a Settings,
        theme: &'b Theme,
    ) -> Box<impl Layout<'a, 'b> + Finalize<'a, 'b>> {
        match self {
            Layouts::Umbrella => Box::new(UmbrellaHtmlTeraRender::new(settings, theme)),
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
