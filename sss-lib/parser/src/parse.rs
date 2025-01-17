use sss_core::Settings;

use crate::error::{Error, Result};
use std::fs;

fn fetch(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}
/// Parse config [Settings] from toml file
pub fn parse_toml(path: &str) -> Result<Settings> {
    let file = fetch(path)?;
    Ok(toml::from_str(&file)?)
}
/// Parse config [Settings] from json file
pub fn parse_json(path: &str) -> Result<Settings> {
    let file = fetch(path)?;
    Ok(serde_json::from_str(&file)?)
}

pub fn parse(path: &str) -> Result<Settings> {
    let aspect = path
        .split(".")
        .last()
        .ok_or(Error::ArgumentIncorrect("path".to_string()))?;
    match aspect {
        "json" => parse_json(path),
        "toml" => parse_toml(path),
        _ => Err(Error::ArgumentIncorrect("path".to_string())),
    }
}
