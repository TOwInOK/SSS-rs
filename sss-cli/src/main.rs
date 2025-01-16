use std::sync::{Arc, LazyLock};

use clap::Parser;
use cli::{
    args::Args,
    subcommands::{self, Commands},
};
use commands::{gen::command_gen, new::command_new, run::command_run};
use render::theme::Theme;
use sss_core::Settings;
use sss_std::themes::Themes;
use tokio::sync::RwLock;

mod cli;
mod commands;
mod tools;
pub mod web;

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
        Commands::New {
            config_type,
        } => command_new(&config_type, &args).await,
        Commands::Run {
            watch,
            serve,
        } => command_run(watch, serve, args.config_path).await,
        Commands::Gen {} => command_gen(&args).await,
    }
}
