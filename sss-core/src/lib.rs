use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use types::{link::Link, skill::Skill, user::User};
pub mod types;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    /// User info
    pub user: User,
    /// Уклон в разработке
    pub specifications: Vec<String>,
    /// О пользователе
    pub about: String,
    /// Репозитории
    pub repos: Vec<Link>,
    /// Социальные сети
    pub socials: Vec<Link>,
    /// Список навыков
    pub skills: Vec<Skill>,
}
