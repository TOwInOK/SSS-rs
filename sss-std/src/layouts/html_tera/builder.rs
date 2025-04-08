//! HTML Tera builder
//!
//! -

use std::borrow::Cow;

use render::{prelude::*, render::FilterLimitations};
use sss_core::Data;
use tera::{Context, Tera};
use theme::Theme;

use crate::layouts::html_tera::html_meta::Meta;

use super::{
    final_builder::HtmlTeraFinalize,
    tools::{TeraData, get_icon_name, get_svg},
};

/// Base Tera component
#[derive(Clone, Debug)]
pub struct HtmlTeraRender<'a, 'b, 'c, L>
where
    L: Layout<String> + Clone,
{
    /// LayoutData
    pub settings: Cow<'a, Data>,
    /// Theme
    pub theme: Cow<'b, Theme>,
    /// Card/Component template
    pub layout: Cow<'c, L>,
}
impl<'a, 'b, 'c, L> HtmlTeraRender<'a, 'b, 'c, L>
where
    L: Layout<String> + Clone,
{
    pub fn new(
        data: &'a Data,
        theme: &'b Theme,
        layout: &'c L,
    ) -> Self {
        Self {
            settings: Cow::Borrowed(data),
            theme: Cow::Borrowed(theme),
            layout: Cow::Borrowed(layout),
        }
    }
    pub fn finalize<FL: Layout<String> + Clone>(
        self,
        layout: &FL,
    ) -> HtmlTeraFinalize<'_, '_, Self, FL> {
        HtmlTeraFinalize {
            meta: Meta::from((&self.settings.layout, self.theme.as_ref())),
            layout: Cow::Borrowed(layout),
            component: Cow::Owned(self),
        }
    }
}

impl<L: Layout + Clone> Layout for HtmlTeraRender<'_, '_, '_, L> {
    fn template(&self) -> Cow<String> {
        self.layout.template()
    }
}

impl<L> Render for HtmlTeraRender<'_, '_, '_, L>
where
    L: Layout<String> + Clone,
    Self: TeraData,
{
    fn render(&self) -> Result<Cow<String>> {
        // Init data
        let theme = self.get_theme();
        let data = &self.filter();
        let mut tera = Tera::default();
        let layout = self.get_layout();

        // Add layout
        tera.add_raw_template("card.html", layout.as_str())?;
        // Register svg return fn
        // it need to use because tera don't use Display/ToString as expected for type
        tera.register_function("get_svg", get_svg);
        // icon to string convertation
        tera.register_function("get_icon_name", get_icon_name);

        // Init context
        let mut context = Context::new();

        // Info about data types
        //
        // User data
        // - user : User
        //  - name : String
        //  - current_nickname : Nickname => word : pronounce
        //  - prevision_nicknames : Vec<Nickname>
        //
        // Aboout you work skills
        // - specifications : Vec<String>
        //
        // Some text info
        // - about : String
        //
        // Type Link :
        //  - provider : Provider
        //  - link : String
        //
        // Links on your repos
        // - repos : Vec<Link>
        // Links on your socials media
        // - socials : Vec<Link>
        //
        // Languages/Staff/Skills of your job
        // - skills : Vec<Skill>
        //
        // Type Skill :
        // - skill : String
        // - projects : Vec<Project>
        // - since : Since => start : usize | end : usize
        // - main : bool
        // - repo_link : Link
        //
        // Type Project :
        // - name : String
        // - link : Link

        // user section

        let user = &data.layout.user;
        // Type Link :
        //  - provider : Provider
        //  - link : String
        context.insert("name", &user.name);
        context.insert("current_nickname", &user.current_nickname);
        context.insert("prevision_nicknames", &user.prevision_nicknames);

        // specifications

        context.insert("specifications", &data.layout.specifications);

        // about

        context.insert("about", &data.layout.about);

        // repos

        context.insert("repos", &data.layout.repos);

        // Type Link :
        //  - provider : Provider
        //  - link : String

        // socials

        context.insert("socials", &data.layout.socials);

        // Type Link :
        //  - provider : Provider
        //  - link : String

        // skills

        context.insert("skills", &data.layout.skills);

        // Type Skill :
        // - skill : String
        // - projects : Vec<Project>
        // - since : Since => start : usize | end : usize
        // - main : bool
        // - repo_link : Link

        // Colors

        context.insert("text", &theme.colors.text);
        context.insert("background", &theme.colors.background);
        context.insert("accent", &theme.colors.accent);
        context.insert("border", &theme.colors.border);

        let rendered = tera.render("card.html", &context)?;
        Ok(Cow::Owned(rendered))
    }
}

impl<L: Layout<String> + Clone> Component<String> for HtmlTeraRender<'_, '_, '_, L> {}

impl<L> Limitations for HtmlTeraRender<'_, '_, '_, L>
where
    L: Layout<String> + Clone,
{
    fn limitations(&self) -> Option<Cow<sss_core::LayoutLimitations>> {
        self.layout.limitations()
    }
}

impl<L: Layout<String> + Clone> FilterLimitations for HtmlTeraRender<'_, '_, '_, L> {
    fn filter(&self) -> Cow<Data> {
        if let Some(limitations) = &self.limitations() {
            let mut data = (*self.settings).clone();

            // Filter user section
            if let Some(user_limits) = limitations.user {
                safe_truncate(&mut data.layout.user.name, user_limits.name_len);
                safe_truncate(
                    &mut data.layout.user.current_nickname.word,
                    user_limits.global_nickname_len,
                );
                safe_truncate(
                    &mut data.layout.user.current_nickname.pronounce,
                    user_limits.global_nickname_pronounce_len,
                );

                if data.layout.user.prevision_nicknames.len()
                    > user_limits.prevision_nicknames_max_count
                {
                    data.layout
                        .user
                        .prevision_nicknames
                        .truncate(user_limits.prevision_nicknames_max_count);
                }

                for nick in &mut data.layout.user.prevision_nicknames {
                    safe_truncate(&mut nick.word, user_limits.global_nickname_len);
                    safe_truncate(
                        &mut nick.pronounce,
                        user_limits.global_nickname_pronounce_len,
                    );
                }
            }

            // Filter specifications
            let (spec_count, spec_len) = limitations.specifications_count;
            if data.layout.specifications.len() > spec_count {
                data.layout.specifications.truncate(spec_count);
            }
            for spec in &mut data.layout.specifications {
                safe_truncate(spec, spec_len);
            }

            // Filter about section
            if let Some(about_len) = limitations.about {
                safe_truncate(&mut data.layout.about, about_len);
            }

            // Filter repositories
            let (repos_count, repos_string_len) = limitations.repositories;
            if data.layout.repos.len() > repos_count {
                data.layout.repos.truncate(repos_count);
            }
            for repo in &mut data.layout.repos {
                safe_truncate(&mut repo.name, repos_string_len);
                safe_truncate(&mut repo.link.link, repos_string_len);
            }

            // Filter socials
            if let Some(socials_count) = limitations.socials {
                if data.layout.socials.len() > socials_count {
                    data.layout.socials.truncate(socials_count);
                }
            }

            // Filter skills
            let (skills_count, skill_limits) = limitations.skills;
            if data.layout.skills.len() > skills_count {
                data.layout.skills.truncate(skills_count);
            }

            for skill in &mut data.layout.skills {
                safe_truncate(&mut skill.skill, skill_limits.skill_len);

                if let Some((proj_count, proj_name_len)) = skill_limits.projects {
                    if skill.projects.len() > proj_count {
                        skill.projects.truncate(proj_count);
                    }
                    for proj in &mut skill.projects {
                        safe_truncate(&mut proj.name, proj_name_len);
                    }
                }

                if !skill_limits.since {
                    skill.since = Default::default();
                }
                if !skill_limits.main {
                    skill.main = false;
                }
                if !skill_limits.repo_link {
                    skill.repo_link = Default::default();
                }
            }

            Cow::Owned(data)
        } else {
            Cow::Borrowed(&self.settings)
        }
    }
}

/// Safe string truncation that counts actual characters (graphemes)
fn safe_truncate(
    s: &mut String,
    max_chars: usize,
) {
    if s.chars().count() > max_chars {
        let mut char_indices = s.char_indices();
        if let Some((idx, _)) = char_indices.nth(max_chars) {
            s.truncate(idx);
        }
    }
}
