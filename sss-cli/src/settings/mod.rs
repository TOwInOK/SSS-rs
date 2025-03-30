use parser::parse::{Loader, Saver};
use serde::{Deserialize, Serialize};
use services::Services;
use sss_core::Data;
use sss_std::prelude::{HtmlLayouts, Themes};

pub mod services;

/// [Settings] wrapper for collecting theme and layout into config for reload on save
#[derive(Debug, Default, Serialize, Deserialize, Clone, utoipa::ToSchema, PartialEq)]
#[serde(rename = "settings")]
pub struct SSSCliSettings {
    /// User specific settings
    #[serde(rename = "user")]
    #[serde(default)]
    pub data: Data,

    /// Theme configuration
    #[serde(rename = "theme")]
    #[serde(default)]
    pub themes: Themes,

    /// Layout configuration
    #[serde(rename = "layout")]
    #[serde(default)]
    pub layouts: HtmlLayouts,

    /// Services to run
    #[serde(rename = "services")]
    #[serde(default)]
    pub services: Services,
}

impl Loader for SSSCliSettings {}
impl Saver for SSSCliSettings {}
