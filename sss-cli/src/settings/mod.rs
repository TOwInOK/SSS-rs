use crate::tools::Result;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use services::Services;
use sss_core::Settings;
use sss_std::prelude::{Layouts, Themes};

pub mod services;

/// [Settings] wrapper for collecting theme and layout into config for reload on save
#[derive(Debug, Default, Serialize, Deserialize, Clone, utoipa::ToSchema, PartialEq)]
#[serde(rename = "settings")]
pub struct SSSCliSettings {
    /// User specific settings
    #[serde(rename = "user")]
    #[serde(default)]
    pub sss_user_settings: Settings,

    /// Theme configuration
    #[serde(rename = "theme")]
    #[serde(default)]
    pub themes: Themes,

    /// Layout configuration
    #[serde(rename = "layout")]
    #[serde(default)]
    pub layouts: Layouts,
    #[serde(rename = "services")]
    #[serde(default)]
    pub services: Services,
}

impl SSSCliSettings {
    /// Load [SSSCliSettings] from path
    #[inline]
    pub async fn load(path: impl AsRef<str>) -> Result<Self> {
        Self::parse(path.as_ref()).await
    }
    /// parse path to [SSSCliSettings]
    #[inline]
    async fn fetch(path: &str) -> Result<String> {
        Ok(tokio::fs::read_to_string(path).await?)
    }
    #[inline]
    /// Parse config [SSSCliSettings] from toml file
    async fn parse_toml(path: &str) -> Result<SSSCliSettings> {
        let file = Self::fetch(path).await?;
        Ok(toml::from_str(&file)?)
    }
    #[inline]
    /// Parse config [SSSCliSettings] from json file
    async fn parse_json(path: &str) -> Result<SSSCliSettings> {
        let file = Self::fetch(path).await?;
        Ok(serde_json::from_str(&file)?)
    }
    #[inline]
    async fn parse(path: &str) -> Result<SSSCliSettings> {
        let aspect = path
            .split(".")
            .last()
            .ok_or(anyhow!("Not found file extension!"))?;
        match aspect {
            "json" => Self::parse_json(path).await,
            "toml" => Self::parse_toml(path).await,
            _ => Err(anyhow!(format!("Unexpected file extension: {}", aspect)).into()),
        }
    }
}
