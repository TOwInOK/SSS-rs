use serde::{Deserialize, Serialize};

use super::{link::Link, since::Since};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Skill {
    /// Specific skill
    pub skill: String,
    /// Project with this skill
    pub projects: Vec<Project>,
    #[serde(default)]
    /// Where start and end
    /// if end is 0 -> today
    pub since: Since,
    #[serde(default)]
    /// if false -> opacity 80%
    pub main: bool,
    /// Link to repository
    /// as example:
    /// - rust -> Crates
    /// - js -> npm
    ///
    /// or you can set
    /// - gh
    /// - gitlab
    pub repo_link: Link,
}

impl parser::prelude::Loader for Skill {}
impl parser::prelude::Saver for Skill {}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Project {
    /// Name of project
    pub name: String,
    /// Link to project
    pub link: Link,
}

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Default, Deserialize, Clone, Copy, PartialEq)]
/// Skill limitations
pub struct SkillLimitation {
    /// Max len of skill [String]
    pub skill_len: usize,
    /// need to contain projects?
    ///
    /// (max len of [Vec], Max len name field peer item)
    pub projects: Option<(usize, usize)>,
    /// can it be since?
    pub since: bool,
    /// can it be main?
    pub main: bool,
    /// can it have a link to repository?
    pub repo_link: bool,
}
