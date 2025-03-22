/// Cli structures for clap
mod cli;
/// functions to run function
mod commands;
/// SSS-rs settings wrapper
pub mod settings;
/// usability functions
pub mod tools;
/// Web utility for serve
pub mod web;

use std::sync::LazyLock;

use clap::Parser;
use cli::{
    args::Args,
    subcommands::{self, Commands},
};
use commands::{geneneration::command_generation, new::command_new, run::command_run};
use settings::SSSCliSettings;
use tokio::sync::RwLock;
use tracing::Level;

/// Init logger
fn init_logger(level: Level) {
    use tracing_subscriber::FmtSubscriber;

    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .compact()
            .with_max_level(level)
            .without_time()
            .finish(),
    )
    .expect("Fail to set global default subscriber");
}

type Llar<T> = LazyLock<RwLock<T>>;

/// Settings pool
pub static SETTINGS: Llar<SSSCliSettings> =
    LazyLock::new(|| RwLock::new(SSSCliSettings::default()));
/// Html pool
pub static HTML: Llar<String> = LazyLock::new(|| RwLock::new(String::new()));
/// HTML
pub static PDF: Llar<Vec<u8>> = LazyLock::new(|| RwLock::new(Vec::new()));
/// Html pool
pub static PNG: Llar<Vec<u8>> = LazyLock::new(|| RwLock::new(Vec::new()));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse args
    let args = Args::parse();
    // Init logger with level
    init_logger((&args.tracing).into());

    match &args.command {
        Commands::New {
            config_type,
            base64,
        } => command_new(config_type, base64.as_ref(), &args).await,
        Commands::Run {
            watch,
            serve,
            address,
            html,
            png,
            pdf,
            json,
            health,
            share,
            api,
        } => {
            command_run(
                *watch,
                *serve,
                &args.config_path,
                &args.layout,
                address,
                &args.theme,
                *html,
                *png,
                *pdf,
                *json,
                *health,
                *share,
                *api,
            )
            .await
        }
        Commands::Gen {
            output_type,
            output_name,
        } => command_generation(output_type, output_name, &args).await,
    }
}
