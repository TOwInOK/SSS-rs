use serde::{Deserialize, Serialize};

use super::nickname::Nickname;
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    /// Your name
    pub name: String,
    /// Nickname
    pub current_nickname: Nickname,
    /// Other nicknames
    pub prevision_nicknames: Vec<Nickname>,
}
