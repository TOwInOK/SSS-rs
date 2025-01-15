use std::error;

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
    pub fn to_layout<'a>(
        &self,
        theme: &'a Theme,
        data: &'a Settings,
    ) -> Box<
        impl Layout<'a, 'a, Result<String, Box<dyn error::Error>>>
            + Finalize<'a, 'a, Result<String, Box<dyn error::Error>>>,
    > {
        match self {
            Layouts::Umbrella => Box::new(UmbrellaHtmlTeraRender {
                data,
                theme,
            }),
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
