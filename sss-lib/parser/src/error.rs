use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("fail to fetch file: {0}")]
    Io(#[from] std::io::Error),
    #[error("fail to parse toml: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("fail to parse json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("fail to parse ron: {0}")]
    Ron(#[from] ron::Error),
    #[error("Argument incorrect: {0}")]
    ArgumentIncorrect(String),
    #[error("fail to save toml: {0}")]
    TomlSave(#[from] toml::ser::Error),
    #[error("Wrong path: {0}")]
    WrongFilePath(String),
}

pub type Result<T> = std::result::Result<T, Error>;
