#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Generate new config
    New {
        #[arg(short, long, default_value_t)]
        config_type: ConfigType,
    },
    /// Run web server
    Run {
        /// reload on save config?
        /// it's linter too.
        #[arg(short, long, default_value_t)]
        watch: bool,
        /// run web server for page
        #[arg(short, long, default_value_t)]
        serve: bool,
        /// address for web server
        #[arg(short, long, default_value_t = default_address())]
        address: String,
    },
    /// Generate html
    Gen {},
}

#[derive(Debug, clap::ValueEnum, Default, Clone, Copy)]
pub enum ConfigType {
    Json,
    #[default]
    Toml,
}

impl std::fmt::Display for ConfigType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ConfigType::Json => write!(f, "json"),
            ConfigType::Toml => write!(f, "toml"),
        }
    }
}

fn default_address() -> String {
    "0.0.0.0:8081".to_string()
}
