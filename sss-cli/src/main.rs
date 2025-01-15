use clap::{command, Parser};

use parser::parse::parse;
use sss_core::Settings;
use sss_std::{prelude::Layouts, themes::Themes};

use std::sync::{Arc, LazyLock, Mutex};

static SETTINGS: LazyLock<Arc<Mutex<Settings>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Settings::default())));

fn refresh_settings(path: &str) {
    let settings = parse(Some(path)).unwrap();
    *SETTINGS.lock().expect("fail to unlock settings") = settings;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    refresh_settings(&args.config_path);

    Ok(())
}

// Take args
// - path config
// - theme
// - layout
// - path output
// - separate css?
// - only component? (no finalyze)
/// SSS-cli is CLI tool for generating html using SSS-rs themes and layouts
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None
)]
struct Args {
    /// path of config
    /// support .json and .toml
    #[arg(short = 'c', long = "config", default_value_t = default_config_path())]
    config_path: String,

    /// Theme choose
    #[arg(short, long, default_value_t = Themes::default())]
    theme: Themes,

    /// Layout choose
    #[arg(short, long, default_value_t = Layouts::default())]
    layout: Layouts,

    /// name or path (without file extension) to file
    #[arg(short = 'o', long = "out", default_value_t = default_file_output())]
    file_output: String,

    /// run web server?
    #[arg(short = 's', long = "serve", default_value_t)]
    serve: bool,

    /// reload on save config?
    #[arg(short = 'w', long = "watch", default_value_t)]
    watch: bool,
}

fn default_file_output() -> String {
    "sss-rs".to_string()
}

fn default_config_path() -> String {
    "config.toml".to_string()
}
