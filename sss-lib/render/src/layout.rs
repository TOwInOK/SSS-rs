use std::{borrow::Cow, fmt::Display};

use sss_core::{LayoutLimitations, types::skill::SkillLimitation};

/// Allow to build a component to string
pub trait Layout<T = String>
where
    T: Clone + Display,
    Self: Limitations,
{
    /// Return the layout as a Cow<str>
    fn template(&self) -> Cow<T>;
}

pub trait Limitations {
    /// Return the layout limitations
    fn limitations(&self) -> Option<Cow<LayoutLimitations>>;

    fn is_user_section_allowed(&self) -> bool {
        self.limitations().is_some_and(|x| x.user.is_some())
    }

    fn is_user_prevision_nicknames_section_allowed(&self) -> bool {
        self.limitations().is_some_and(|x| {
            x.user
                .as_ref()
                .is_some_and(|x| x.prevision_nicknames_max_count > 0)
        })
    }

    fn is_specification_section_allowed(&self) -> bool {
        self.limitations()
            .is_some_and(|x| x.specifications_count.0 > 0)
    }

    fn is_about_section_allowed(&self) -> bool {
        self.limitations()
            .is_some_and(|x| x.about.is_some_and(|x| x > 0))
    }
    fn is_repositories_section_allowed(&self) -> bool {
        self.limitations().is_some_and(|x| x.repositories.0 > 0)
    }

    fn is_socials_section_allowed(&self) -> bool {
        self.limitations()
            .is_some_and(|x| x.socials.is_some_and(|x| x > 0))
    }

    fn is_projects_in_skills_allowed(&self) -> bool {
        self.limitations()
            .is_some_and(|x| x.skills.0 > 0 && x.skills.1.projects.is_some())
    }

    fn is_skills_section_allowed(&self) -> bool {
        self.limitations().is_some_and(|x| x.skills.0 > 0)
    }

    fn user_name_len(&self) -> usize {
        self.limitations()
            .as_ref()
            .and_then(|x| x.user.as_ref())
            .map(|user_limitations| user_limitations.name_len)
            .unwrap_or_default()
    }

    fn user_global_nickname_len(&self) -> usize {
        self.limitations()
            .as_ref()
            .and_then(|x| x.user.as_ref())
            .map(|user_limitations| user_limitations.global_nickname_len)
            .unwrap_or_default()
    }

    fn user_global_nickname_pronounce_len(&self) -> usize {
        self.limitations()
            .as_ref()
            .and_then(|x| x.user.as_ref())
            .map(|user_limitations| user_limitations.global_nickname_pronounce_len)
            .unwrap_or_default()
    }

    fn user_prevision_nicknames_max_count(&self) -> usize {
        self.limitations()
            .as_ref()
            .and_then(|x| x.user.as_ref())
            .map(|user_limitations| user_limitations.prevision_nicknames_max_count)
            .unwrap_or_default()
    }

    fn specifications_count(&self) -> (usize, usize) {
        self.limitations()
            .as_ref()
            .map(|x| x.specifications_count)
            .unwrap_or_default()
    }

    fn about(&self) -> usize {
        self.limitations()
            .as_ref()
            .and_then(|x| x.about)
            .unwrap_or_default()
    }

    fn repositories(&self) -> (usize, usize) {
        self.limitations()
            .as_ref()
            .map(|x| x.repositories)
            .unwrap_or_default()
    }

    fn repositories_max_len(&self) -> usize {
        self.limitations()
            .as_ref()
            .map(|x| x.repositories.0)
            .unwrap_or_default()
    }

    fn repositories_max_string_len(&self) -> usize {
        self.limitations()
            .as_ref()
            .map(|x| x.repositories.1)
            .unwrap_or_default()
    }

    fn socials(&self) -> usize {
        self.limitations()
            .as_ref()
            .and_then(|x| x.socials)
            .unwrap_or_default()
    }

    fn skills(&self) -> (usize, SkillLimitation) {
        self.limitations()
            .as_ref()
            .map(|x| x.skills)
            .unwrap_or_default()
    }

    fn skills_max_len(&self) -> usize {
        self.limitations()
            .as_ref()
            .map(|x| x.skills.0)
            .unwrap_or_default()
    }

    fn skills_projects(&self) -> (usize, usize) {
        self.limitations()
            .as_ref()
            .and_then(|x| x.skills.1.projects)
            .unwrap_or_default()
    }

    fn skills_skill_len(&self) -> usize {
        self.limitations()
            .as_ref()
            .map(|x| x.skills.1.skill_len)
            .unwrap_or_default()
    }

    fn skills_since(&self) -> bool {
        self.limitations()
            .as_ref()
            .map(|x| x.skills.1.since)
            .unwrap_or_default()
    }

    fn skills_main(&self) -> bool {
        self.limitations()
            .as_ref()
            .map(|x| x.skills.1.main)
            .unwrap_or_default()
    }

    fn skills_repo_link(&self) -> bool {
        self.limitations()
            .as_ref()
            .map(|x| x.skills.1.repo_link)
            .unwrap_or_default()
    }
}
