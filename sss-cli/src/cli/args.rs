use std::fmt::Display;

use clap::{Parser, command};
use sss_std::{prelude::HtmlLayouts, themes::Themes};
use tracing::Level;

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
    /// name of config
    /// support .json and .toml
    #[arg(short = 'c', long = "config", default_value_t = default_config_path())]
    pub config_path: String,

    /// Theme choose
    #[arg(short, long)]
    pub theme: Option<Themes>,

    /// Layout choose
    #[arg(short, long)]
    pub layout: Option<HtmlLayouts>,

    /// Log level
    #[arg(long, default_value_t)]
    pub tracing: CliTracing,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, clap::ValueEnum, Default)]
pub enum CliTracing {
    #[default]
    Info,
    Trace,
    Debug,
    Error,
    Warn,
}

impl Display for CliTracing {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            CliTracing::Info => write!(f, "info"),
            CliTracing::Trace => write!(f, "trace"),
            CliTracing::Debug => write!(f, "debug"),
            CliTracing::Error => write!(f, "error"),
            CliTracing::Warn => write!(f, "warn"),
        }
    }
}

impl From<&CliTracing> for tracing::Level {
    fn from(value: &CliTracing) -> Self {
        match value {
            CliTracing::Info => Level::INFO,
            CliTracing::Trace => Level::TRACE,
            CliTracing::Debug => Level::DEBUG,
            CliTracing::Error => Level::ERROR,
            CliTracing::Warn => Level::WARN,
        }
    }
}

pub fn default_config_path() -> String {
    "config.toml".to_string()
}
