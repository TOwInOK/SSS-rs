use serde::{Deserialize, Serialize};

use super::nickname::Nickname;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    /// Your name
    pub name: String,
    /// Nickname
    pub current_nickname: Nickname,
    /// Other nicknames
    pub prevision_nicknames: Vec<Nickname>,
}
