use crate::error::Result;
use crate::types::User;
use std::fs;

pub const DEFAULT_USER_PATH_TOML: &str = "./user.toml";
pub const DEFAULT_USER_PATH_JSON: &str = "./user.json";

fn fetch(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn parse_toml(path: Option<&str>) -> Result<User> {
    if let Some(path) = path {
        let file = fetch(path)?;
        return Ok(toml::from_str(&file)?);
    }
    let file = fetch(DEFAULT_USER_PATH_TOML)?;
    Ok(toml::from_str(&file)?)
}

pub fn parse_json(path: Option<&str>) -> Result<User> {
    if let Some(path) = path {
        let file = fetch(path)?;
        return Ok(serde_json::from_str(&file)?);
    }
    let file = fetch(DEFAULT_USER_PATH_JSON)?;
    Ok(serde_json::from_str(&file)?)
}
