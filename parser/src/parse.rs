use sss_core::Settings;

use crate::error::{Error, Result};
use std::fs;

fn fetch(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}
/// Parse config [Settings] from toml file
pub fn parse_toml(path: Option<&str>) -> Result<Settings> {
    if let Some(path) = path {
        let file = fetch(path)?;
        Ok(toml::from_str(&file)?)
    } else {
        Err(Error::ArgumentIncorrect("path".to_string()))
    }
}
/// Parse config [Settings] from json file
pub fn parse_json(path: Option<&str>) -> Result<Settings> {
    if let Some(path) = path {
        let file = fetch(path)?;
        Ok(serde_json::from_str(&file)?)
    } else {
        Err(Error::ArgumentIncorrect("path".to_string()))
    }
}

pub fn parse(path: Option<&str>) -> Result<Settings> {
    if let Some(path) = path {
        let aspect = path
            .split(".")
            .last()
            .ok_or(Error::ArgumentIncorrect("path".to_string()))?;
        match aspect {
            "json" => parse_json(Some(path)),
            "toml" => parse_toml(Some(path)),
            _ => Err(Error::ArgumentIncorrect("path".to_string())),
        }
    } else {
        Err(Error::ArgumentIncorrect("path".to_string()))
    }
}
