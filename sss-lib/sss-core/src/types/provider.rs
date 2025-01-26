use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

const GITHUB_OTLINED: &str = include_str!("provider/github_ontlined.svg");
const LINKED_IN_OTLINED: &str = include_str!("provider/linkedin_outlined.svg");
const TELEGRAM_OTLINED: &str = include_str!("provider/telegram_ontlined.svg");

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
/// Icon provider
pub enum Provider {
    #[default]
    Github,
    LinkedIn,
    Telegram,
}

impl Provider {
    pub fn all_providers() -> Vec<Provider> {
        vec![Self::Github, Self::LinkedIn, Self::Telegram]
    }
}

impl FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" | "gh" => Ok(Self::Github),
            "linkedin" | "l.in" => Ok(Self::LinkedIn),
            "telegram" | "tg" => Ok(Self::Telegram),
            _ => Err(format!("undefined {}", s)),
        }
    }
}

impl AsRef<str> for Provider {
    /// Return svg file
    fn as_ref(&self) -> &str {
        match self {
            Provider::Github => GITHUB_OTLINED,
            Provider::LinkedIn => LINKED_IN_OTLINED,
            Provider::Telegram => TELEGRAM_OTLINED,
        }
    }
}
impl Display for Provider {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Provider::Github => write!(f, "Github"),
            Provider::LinkedIn => write!(f, "LinkedIn"),
            Provider::Telegram => write!(f, "Telegram"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_svg() {
        let provider = Provider::Github;
        println!("{}", provider.as_ref())
    }
}
