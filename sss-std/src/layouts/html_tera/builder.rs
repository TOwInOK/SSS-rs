//! HTML Tera builder
//!
//! -

use std::borrow::Cow;

use render::prelude::*;
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
        let settings = self.get_data();
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

        let user = &settings.layout.user;
        // Type Link :
        //  - provider : Provider
        //  - link : String
        context.insert("name", &user.name);
        context.insert("current_nickname", &user.current_nickname);
        context.insert("prevision_nicknames", &user.prevision_nicknames);

        // specifications

        context.insert("specifications", &settings.layout.specifications);

        // about

        context.insert("about", &settings.layout.about);

        // repos

        context.insert("repos", &settings.layout.repos);

        // Type Link :
        //  - provider : Provider
        //  - link : String

        // socials

        context.insert("socials", &settings.layout.socials);

        // Type Link :
        //  - provider : Provider
        //  - link : String

        // skills

        context.insert("skills", &settings.layout.skills);

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
