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

use std::sync::{Arc, LazyLock};

use clap::Parser;
use cli::{
    args::Args,
    subcommands::{self, Commands},
};
use commands::{gen::command_gen, new::command_new, run::command_run};
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

/// Settings pool
pub static SETTINGS: LazyLock<Arc<RwLock<SSSCliSettings>>> =
    LazyLock::new(|| Arc::new(RwLock::new(SSSCliSettings::default())));
/// Html pool
pub static HTML: LazyLock<Arc<RwLock<String>>> =
    LazyLock::new(|| Arc::new(RwLock::new(String::new())));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse args
    let args = Args::parse();
    // Init logger with level
    init_logger((&args.tracing).into());

    match args.command {
        Commands::New {
            config_type,
        } => command_new(&config_type, &args).await,
        Commands::Run {
            watch,
            serve,
            address,
        } => {
            command_run(
                watch,
                serve,
                &args.config_path,
                &args.layout,
                &address,
                &args.theme,
            )
            .await
        }
        Commands::Gen {} => command_gen(&args).await,
    }
}
