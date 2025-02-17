#[derive(
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    utoipa::ToSchema,
    PartialEq,
    clap::Parser,
)]
#[serde(rename = "settings")]
/// Choose services to launch
pub struct Services {
    /// launch html service
    pub html: bool,
    /// launch png generator service
    pub png: bool,
    /// launch pdf generator service
    pub pdf: bool,
    /// launch json converter for settings service
    pub json: bool,
    /// launch healf checker service
    pub health: bool,
    /// launch base64 service
    pub share: bool,
    /// launch rapidoc api service
    pub api: bool,
}
