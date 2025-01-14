use clap::{command, Parser};

use sss_std::{prelude::UmbrellaHtmlRender, themes::*};
use std::{fmt::Display, fs, str::FromStr};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file = args.file_output;
    let theme = match &args.theme {
        Themes::Umbrella => &UMBRELLA,
        Themes::RosePine => &ROSE_PINE,
        Themes::Groovebox => &GROOVEBOX,
        Themes::Dracula => &DRACULA,
    };
    if args.config_path.is_empty() {
        panic!("need file path for config");
    }
    // let config = match &args.config_path.split(".").last() {
    //     Some(e) => match *e {
    //         "toml" => parse_toml(Some(e)),
    //         "json" => parse_json(Some(e)),
    //         _ => panic!("unsuported extension: {e}"),
    //     },
    //     None => panic!("not found file extension?"),
    // }?
    // let rendered = match &args.layout {
    //     Layouts::Umbrella => {
    //         let ub = UmbrellaHtmlRender;
    //         ub.render(config, theme)
    //     }
    // };
    // let ub = UmbrellaHtmlRender;
    // if !args.is_component {
    //     let html = ub.finalize(rendered, theme).map_err(|x| anyhow!(x.to_string()))?;
    //     fs::write(format!("{file}.html"), html)?;
    // } else {
    //     let html = rendered.map_err(|x| anyhow!(x.to_string()))?;

    //     fs::write(format!("{file}.html"), html)?;
    // };
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

    // /// make file with css (not include css into html file)
    // #[arg(short = 's', long = "separate", default_value_t = CssTunes::default())]
    // separate_css: CssTunes,
    /// Build only component (not finalyze)
    #[arg(short = 'i', long = "ict", default_value_t = false)]
    is_component: bool,
}

fn default_file_output() -> String {
    "sss-rs".to_string()
}

fn default_config_path() -> String {
    "config.toml".to_string()
}

#[derive(Debug, Clone, Default)]
enum CssTunes {
    /// Build in file
    /// name.html <-- name.css (like style)
    #[default]
    Together,
    /// name.html + name.css
    Separate,
    /// only name.html without css converation
    None,
}

impl Display for CssTunes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssTunes::Together => write!(f, "together"),
            CssTunes::Separate => write!(f, "separate"),
            CssTunes::None => write!(f, "none"),
        }
    }
}
impl FromStr for CssTunes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "together" => Ok(Self::Together),
            "separate" => Ok(Self::Separate),
            "none" => Ok(Self::None),
            _ => Err(format!("Not found Css Moves! -> {s}")),
        }
    }
}

#[derive(Debug, Clone, Default)]
enum Themes {
    #[default]
    Umbrella,
    RosePine,
    Groovebox,
    Dracula,
}

impl Display for Themes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Themes::Umbrella => write!(f, "umbrella"),
            Themes::RosePine => write!(f, "rosepine"),
            Themes::Groovebox => write!(f, "groovebox"),
            Themes::Dracula => write!(f, "dracula"),
        }
    }
}

impl FromStr for Themes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "umbrella" => Ok(Self::Umbrella),
            "rosepine" => Ok(Self::RosePine),
            "groovebox" => Ok(Self::Groovebox),
            "dracula" => Ok(Self::Dracula),
            _ => Err(format!("Not found theme! -> {}", s)),
        }
    }
}

#[derive(Debug, Clone, Default)]
enum Layouts {
    #[default]
    Umbrella,
}

impl Display for Layouts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layouts::Umbrella => write!(f, "umbrella"),
        }
    }
}

impl FromStr for Layouts {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "umbrella" => Ok(Self::Umbrella),
            _ => Err(format!("not found layout! -> {s}")),
        }
    }
}
