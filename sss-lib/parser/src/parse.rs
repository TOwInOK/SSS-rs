use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use std::{fs, path::Path, str::FromStr};
/// Fetch file by path
pub fn fetch(path: &Path) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}
/// Parse config [Settings] from toml file
pub fn parse_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let file = fetch(path)?;
    Ok(toml::from_str(&file)?)
}
/// Parse config [Settings] from json file
pub fn parse_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let file = fetch(path)?;
    Ok(serde_json::from_str(&file)?)
}

pub fn save<T: Serialize>(
    path: &Path,
    data: &T,
) -> Result<()> {
    match FetchType::from_str(path.to_str().unwrap_or_default())? {
        FetchType::Json => save_json(path, data),
        FetchType::Toml => save_toml(path, data),
        FetchType::Ron => save_ron(path, data),
    }
}

pub fn save_toml<T: Serialize>(
    path: &Path,
    data: &T,
) -> Result<()> {
    let file = toml::to_string(data)?;
    fs::write(path, file).map_err(Error::from)
}

pub fn save_ron<T: Serialize>(
    path: &Path,
    data: &T,
) -> Result<()> {
    let file = ron::to_string(data)?;
    fs::write(path, file).map_err(Error::from)
}

pub fn save_json<T: Serialize>(
    path: &Path,
    data: &T,
) -> Result<()> {
    let file = serde_json::to_string_pretty(data)?;
    fs::write(path, file).map_err(Error::from)
}

/// Parse config [Settings] from ron file
pub fn parse_ron<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let file = fetch(path)?;
    ron::from_str(&file).map_err(|e| Error::from(ron::Error::from(e)))
}

/// Fetch [Settings] by path
pub fn parse<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    match FetchType::from_str(path.to_str().unwrap_or_default())? {
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
    fn load<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
        parse(path)
    }
}

pub trait Saver {
    /// Save any T to file
    fn save<T: Serialize>(
        path: &Path,
        data: &T,
    ) -> Result<()> {
        save(path, data)
    }
}
