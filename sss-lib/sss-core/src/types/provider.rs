use std::fmt::Display;

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

impl AsRef<str> for Provider {
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
        let icon = self.as_ref();
        match self {
            Provider::Github => write!(f, "{}", icon),
            Provider::LinkedIn => write!(f, "{}", icon),
            Provider::Telegram => write!(f, "{}", icon),
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
