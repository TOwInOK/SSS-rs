use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use types::{
    link::Link,
    skill::{Project, Skill},
    user::User,
};

pub mod types;

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
/// SSS-rs settings
pub struct Settings {
    /// User info
    pub user: User,
    /// Job specific
    pub specifications: Vec<String>,
    /// About User
    pub about: String,
    /// Repositories
    pub repos: Vec<Project>,
    /// Socials
    pub socials: Vec<Link>,
    /// work skills
    pub skills: Vec<Skill>,
}
