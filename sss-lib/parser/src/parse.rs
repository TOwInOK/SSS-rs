use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use std::{fs, path::Path, str::FromStr};
/// Fetch file by path
pub fn fetch(path: impl AsRef<Path>) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}
/// Parse config [Settings] from toml file
pub fn parse_toml<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T> {
    let file = fetch(path)?;
    Ok(toml::from_str(&file)?)
}
/// Parse config [Settings] from json file
pub fn parse_json<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T> {
    let file = fetch(path)?;
    Ok(serde_json::from_str(&file)?)
}

pub fn save<T: Serialize>(
    path: impl AsRef<Path>,
    data: &T,
) -> Result<()> {
    match FetchType::from_str(
        path.as_ref()
            .to_str()
            .ok_or_else(|| Error::WrongFilePath(path.as_ref().display().to_string()))?,
    )? {
        FetchType::Json => save_json(path, data),
        FetchType::Toml => save_toml(path, data),
        FetchType::Ron => save_ron(path, data),
    }
}

pub fn save_toml<T: Serialize>(
    path: impl AsRef<Path>,
    data: &T,
) -> Result<()> {
    let file = toml::to_string(data)?;
    fs::write(path, file).map_err(Error::from)
}

pub fn save_ron<T: Serialize>(
    path: impl AsRef<Path>,
    data: &T,
) -> Result<()> {
    let file = ron::ser::to_string_pretty(data, PrettyConfig::default())?;
    fs::write(path, file).map_err(Error::from)
}

pub fn save_json<T: Serialize>(
    path: impl AsRef<Path>,
    data: &T,
) -> Result<()> {
    let file = serde_json::to_string_pretty(data)?;
    fs::write(path, file).map_err(Error::from)
}

/// Parse config [Settings] from ron file
pub fn parse_ron<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T> {
    let file = fetch(path)?;
    ron::from_str(&file).map_err(|e| Error::from(ron::Error::from(e)))
}

/// Fetch [Settings] by path
pub fn parse<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T> {
    let path_str = path
        .as_ref()
        .to_str()
        .ok_or_else(|| Error::WrongFilePath(path.as_ref().display().to_string()))?;

    match FetchType::from_str(path_str)? {
        FetchType::Json => parse_json(path),
        FetchType::Toml => parse_toml(path),
        FetchType::Ron => parse_ron(path),
    }
}

#[derive(Debug, Default)]
pub enum FetchType {
    Json,
    #[default]
    Toml,
    Ron,
}

impl FromStr for FetchType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s
            .split(".")
            .last()
            .ok_or(Error::ArgumentIncorrect(format!("Path: {}", s)))?
        {
            "json" => Ok(FetchType::Json),
            "toml" => Ok(FetchType::Toml),
            "ron" => Ok(FetchType::Ron),
            _ => Err(Error::ArgumentIncorrect(format!("Path: {}", s))),
        }
    }
}

pub trait Loader {
    /// Load any T from file
    fn load<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T> {
        parse(path)
    }
}

pub trait Saver {
    /// Save any T to file
    fn save<T: Serialize>(
        path: impl AsRef<Path>,
        data: &T,
    ) -> Result<()> {
        save(path, data)
    }
}
