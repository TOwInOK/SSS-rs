use serde::{Deserialize, Serialize};

use super::nickname::Nickname;
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
/// User data
pub struct User {
    /// Your name
    pub name: String,
    /// Nickname
    pub current_nickname: Nickname,
    /// Other nicknames
    pub prevision_nicknames: Vec<Nickname>,
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Default, Deserialize, Clone, Copy, PartialEq)]
/// Limitations for [User] section
pub struct UserLimitations {
    /// Max len of [String]
    pub name_len: usize,
    /// Max len of all nicknames
    pub global_nickname_len: usize,
    /// Max len of all nicknames pronounces
    pub global_nickname_pronounce_len: usize,
    /// max len of prevision_nicknames [Vec]
    pub prevision_nicknames_max_count: usize,
}
