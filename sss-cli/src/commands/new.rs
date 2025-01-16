use tokio::fs;
use tracing::{info, instrument};

use crate::{
    cli::{args::Args, subcommands::ConfigType},
    tools::gen_example_config,
};

#[instrument(skip(args, config_type))]
pub async fn command_new(
    config_type: &ConfigType,
    args: &Args,
) -> anyhow::Result<()> {
    info!("Start generating new config");
    let settings = gen_example_config();
    let path_to_settings = &args.config_path;
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
    info!("Try to save config by path: {}", path_to_settings);
    fs::write(&path_to_settings, config).await?;
    info!("Config successfully saved by path: {}", path_to_settings);
    Ok(())
}
