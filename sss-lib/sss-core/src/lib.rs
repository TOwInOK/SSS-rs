use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use types::{
    link::Link,
    skill::{Project, Skill, SkillLimitation},
    user::{User, UserLimitations},
};

pub mod types;

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
/// SSS-data
pub struct Data {
    /// Information about user
    pub layout: LayoutData,
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
/// SSS-rs settings
/// Layout data
pub struct LayoutData {
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

impl parser::prelude::Loader for LayoutData {}
impl parser::prelude::Saver for LayoutData {}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Default, Deserialize, Clone, Copy, PartialEq)]
/// Limitations for layout data
pub struct LayoutLimitations {
    /// User section
    ///
    pub user: Option<UserLimitations>,
    /// specifications
    ///
    /// (max len if array, max len of string peer item)
    pub specifications_count: (usize, usize),
    /// About sections
    ///
    /// max len of about [String]
    pub about: Option<usize>,
    /// Repositories section
    ///
    /// (max len if array, Max len item field peer item)
    pub repositories: (usize, usize),
    /// Socials sections
    ///
    /// (max len if array)
    pub socials: Option<usize>,
    /// Skills section
    ///
    /// (max len if array, item settings)
    pub skills: (usize, SkillLimitation),
}
