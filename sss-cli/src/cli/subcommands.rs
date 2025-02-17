#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Generate new config
    New {
        /// Type of config
        #[arg(short = 't', long = "type", default_value_t)]
        config_type: ConfigType,
        /// Base64 converted config (only toml)
        #[arg(short = 'b', long = "base64")]
        base64: Option<String>,
    },
    /// Run web server or/and linter
    Run {
        /// reload on save config?
        ///
        /// Linter
        #[arg(short, long, default_value_t)]
        watch: bool,
        /// run web server
        #[arg(short, long, default_value_t)]
        serve: bool,
        /// address for web server
        #[arg(short, long, default_value_t = default_address())]
        address: String,
        /// launch html service
        #[arg(long)]
        html: bool,
        /// launch png generator service
        #[arg(long)]
        png: bool,
        /// launch pdf generator service
        #[arg(long)]
        pdf: bool,
        /// launch json converter for settings service
        #[arg(long)]
        json: bool,
        /// launch healf checker service
        #[arg(long)]
        health: bool,
        /// launch base64 service
        #[arg(long)]
        share: bool,
        /// launch rapidoc api service
        #[arg(long)]
        api: bool,
    },
    /// Generate file
    Gen {
        /// output type
        #[arg(short = 't', long = "type", default_value_t)]
        output_type: GenType,
        /// output name
        ///
        /// [out].[type]
        ///
        /// by default: sss-rs.html
        #[arg(short = 'o', long = "out", default_value_t = default_gen_out_name())]
        output_name: String,
    },
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

fn default_gen_out_name() -> String {
    "sss-rs".to_string()
}

#[derive(Debug, clap::ValueEnum, Default, Clone, Copy)]
pub enum GenType {
    #[default]
    Html,
    Png,
    Pdf,
}

impl std::fmt::Display for GenType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            GenType::Html => write!(f, "html"),
            GenType::Png => write!(f, "png"),
            GenType::Pdf => write!(f, "pdf"),
        }
    }
}
