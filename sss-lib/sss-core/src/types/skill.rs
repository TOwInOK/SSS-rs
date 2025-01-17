use serde::{Deserialize, Serialize};

use super::{link::Link, since::Since};

#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    /// Name of project
    pub name: String,
    /// Link to project
    pub link: Link,
}
