use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("fail to fetch file. File is exist?")]
    Io(#[from] std::io::Error),
    #[error("fail to parse toml {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("fail to parse json")]
    JsonParse(#[from] serde_json::Error),
    #[error("Argument incorrect: {0}")]
    ArgumentIncorrect(String),
}

pub type Result<T> = std::result::Result<T, Error>;
