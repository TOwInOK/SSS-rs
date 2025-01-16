use clap::{command, Parser};
use sss_std::{prelude::Layouts, themes::Themes};

use crate::subcommands::Commands;

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
pub struct Args {
    /// path of config
    /// support .json and .toml
    #[arg(short = 'c', long = "config", default_value_t = default_config_path())]
    pub config_path: String,

    /// Theme choose
    #[arg(short, long, default_value_t = Themes::default())]
    pub theme: Themes,

    /// Layout choose
    #[arg(short, long, default_value_t = Layouts::default())]
    pub layout: Layouts,

    /// name of file_name.html
    #[arg(short = 'o', long = "out", default_value_t = default_file_output())]
    pub file_output: String,

    #[command(subcommand)]
    pub command: Commands,
}

fn default_file_output() -> String {
    "sss-rs.html".to_string()
}

pub fn default_config_path() -> String {
    "config.toml".to_string()
}
