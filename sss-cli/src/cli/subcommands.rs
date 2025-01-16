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
        #[arg(short, long, default_value_t)]
        watch: bool,
        /// run web page shower?
        #[arg(short, long, default_value_t)]
        serve: bool,
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
