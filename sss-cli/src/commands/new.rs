use tokio::fs;
use tracing::{info, instrument};

use crate::{
    cli::{args::Args, subcommands::ConfigType},
    tools::gen_example_config,
};

#[instrument(skip_all)]
pub async fn command_new(
    config_type: &ConfigType,
    base64: Option<&String>,
    args: &Args,
) -> anyhow::Result<()> {
    info!("Start generating new config");
    let settings = match base64 {
        Some(e) => toml::from_str(&base64_light::base64_decode_str(e))?,
        None => gen_example_config(),
    };
    let mut path_to_settings = std::path::PathBuf::from(&args.config_path);
    path_to_settings.set_extension(config_type.to_string());
    info!("Convert to choosd type");
    let config = match config_type {
        ConfigType::Json => {
            info!("Convert to JSON");
            serde_json::to_string_pretty(&settings).map_err(|x| {
                anyhow::anyhow!(
                    "Got error with generating settings to json type {}",
                    x.to_string()
                )
            })?
        }
        ConfigType::Toml => {
            info!("Convert to TOML");
            toml::to_string_pretty(&settings).map_err(|x| {
                anyhow::anyhow!(
                    "Got error with generating settings to toml type {}",
                    x.to_string()
                )
            })?
        }
    };
    info!("Try to save config by path: {}", path_to_settings.display());
    fs::write(&path_to_settings, config).await?;
    info!("Config successfully saved by path: {}", path_to_settings.display());
    Ok(())
}
