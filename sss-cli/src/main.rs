mod args;
mod subcommands;
mod tools;

use anyhow::anyhow;

use args::Args;
use clap::Parser;
use render::theme::Theme;
use sss_core::Settings;
use sss_std::themes::Themes;
use subcommands::ConfigType;
use tokio::{fs, sync::RwLock};
use tools::{gen_example_config, layout_build, load};
use tracing::{info, instrument};

use std::sync::{Arc, LazyLock};

static TS: LazyLock<()> = LazyLock::new(|| {
    use tracing_subscriber::FmtSubscriber;

    tracing::subscriber::set_global_default(
        FmtSubscriber::builder().compact().without_time().finish(),
    )
    .expect("Fail to set global default subscriber");
});

pub static SETTINGS: LazyLock<Arc<RwLock<Settings>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Settings::default())));

pub static THEME: LazyLock<Arc<RwLock<&'static Theme>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Themes::default().into())));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LazyLock::force(&TS);
    // Parse args
    let args = Args::parse();

    match args.command {
        subcommands::Commands::New {
            config_type,
        } => command_new(&config_type, &args).await,
        subcommands::Commands::Run { watch } => todo!(),
        subcommands::Commands::Gen {} => command_gen(&args).await,
    }
}

#[instrument(skip(args, config_type))]
async fn command_new(
    config_type: &ConfigType,
    args: &Args,
) -> anyhow::Result<()> {
    info!("Start generating new config");
    let settings = gen_example_config();
    let path_to_settings = &args.config_path;
    info!("Convert to choosd type");
    let config = match config_type {
        subcommands::ConfigType::Json => {
            info!("Convert to JSON");
            serde_json::to_string_pretty(&settings).map_err(|x| {
                anyhow!(
                    "Got error with generating settings to json type {}",
                    x.to_string()
                )
            })?
        }
        subcommands::ConfigType::Toml => {
            info!("Convert to TOML");
            toml::to_string_pretty(&settings).map_err(|x| {
                anyhow!(
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
#[instrument(skip(args))]
async fn command_gen(args: &Args) -> anyhow::Result<()> {
    info!("Start generating html");
    info!("Start loading config & theme");
    load(&args.config_path, &args.theme).await;
    info!("Load successfully!");
    info!("Try to generate html");
    let html = layout_build(&args.layout)
        .await
        .map_err(|e| anyhow!("Layout build error: {}", e))?;
    info!("Html generated successfully!");
    info!("Try to write html on disk by path: {}", &args.file_output);
    fs::write(&args.file_output, &html).await?;
    info!("Html successfully saved by path: {}", &args.file_output);
    Ok(())
}
