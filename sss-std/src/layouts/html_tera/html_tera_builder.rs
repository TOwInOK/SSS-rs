//! HTML Tera builder
//!
//! -

use render::prelude::*;
use sss_core::{Settings, types::provider::Tabler};
use tera::{Context, Tera};
use theme::{Shade, Theme};

use crate::tools::gen_css;

use crate::layouts::html_tera::html_meta::Meta;

/// Base Tera component
#[derive(Clone, Debug)]
pub struct HtmlTeraRender<'a> {
    /// Settings
    pub settings: &'a Settings,
    /// Theme
    pub theme: &'static Theme,
    /// Card/Component template
    pub template: &'static str,
}
impl<'a> HtmlTeraRender<'a> {
    pub fn new(
        settings: &'a Settings,
        theme: &'static Theme,
        card_template: &'static str,
    ) -> Self {
        Self {
            settings,
            theme,
            template: card_template,
        }
    }
    pub fn finalize(
        self,
        template: &'static str,
    ) -> HtmlTeraFinalize<Self> {
        HtmlTeraFinalize {
            template,
            meta: Meta::from((self.settings, self.theme)),
            component: self,
        }
    }
}
impl Layout for HtmlTeraRender<'_> {
    fn render(&self) -> Result<String> {
        // Init data
        let theme = self.get_theme();
        let settings = self.get_data();
        let mut tera = Tera::default();
        let layout = self.get_layout();

        // Add layout
        tera.add_raw_template("card.html", layout)?;
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

        let user = &settings.user;
        // Type Link :
        //  - provider : Provider
        //  - link : String
        context.insert("name", &user.name);
        context.insert("current_nickname", &user.current_nickname);
        context.insert("prevision_nicknames", &user.prevision_nicknames);

        // specifications

        context.insert("specifications", &settings.specifications);

        // about

        context.insert("about", &settings.about);

        // repos

        context.insert("repos", &settings.repos);

        // Type Link :
        //  - provider : Provider
        //  - link : String

        // socials

        context.insert("socials", &settings.socials);

        // Type Link :
        //  - provider : Provider
        //  - link : String

        // skills

        context.insert("skills", &settings.skills);

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
        Ok(rendered)
    }
}

/// Wrapper for Tera component [HtmlTeraRender]
/// It's wrap component into Html document
pub struct HtmlTeraFinalize<L>
where
    L: Layout,
{
    /// Finalize template
    pub template: &'static str,
    pub meta: Meta,
    pub component: L,
}

impl<L: Layout + TeraData> Layout for HtmlTeraFinalize<L> {
    fn render(&self) -> render::Result<String> {
        // render card
        let card = self.component.render()?;
        // render css
        let css = gen_css(&self.get_theme().get_encre_css_config(), &card);
        // Init tera
        let mut tera = Tera::default();
        // Add template
        tera.add_raw_template("layout.html", self.get_layout())?;
        // Init context
        let mut context = Context::new();

        // card

        context.insert("card", &card);

        // style

        context.insert("style", &css);

        // fonts

        context.insert("font", &self.get_theme().font().1);

        // meta

        self.meta.insert_to_context(&mut context);

        let rendered = tera.render("layout.html", &context)?;
        Ok(rendered)
    }
}

/// Tera data simplification
pub trait TeraData {
    /// Get data to render
    fn get_data(&self) -> &Settings;
    /// Get theme to theming
    fn get_theme(&self) -> &Theme;
    /// Return layout
    fn get_layout(&self) -> &str;
}

impl TeraData for HtmlTeraRender<'_> {
    #[inline]
    fn get_data(&self) -> &Settings {
        self.settings
    }

    #[inline]
    fn get_theme(&self) -> &Theme {
        self.theme
    }

    #[inline]
    fn get_layout(&self) -> &str {
        self.template
    }
}

impl<L: Layout + TeraData> TeraData for HtmlTeraFinalize<L> {
    #[inline]
    fn get_data(&self) -> &Settings {
        self.component.get_data()
    }

    #[inline]
    fn get_theme(&self) -> &Theme {
        self.component.get_theme()
    }

    #[inline]
    fn get_layout(&self) -> &str {
        self.template
    }
}

/// Return svg [String] by icon name
/// ```tera
///  {{ get_svg(icon=social.icon) | safe }}
/// ```
fn get_svg(args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    // Получаем значение provider из аргументов
    let provider = match args.get("icon") {
        Some(val) => val.to_string(),
        None => {
            return Err(tera::Error::msg(
                "Icon argument is required. Check your function invocation in template",
            ));
        }
    };

    // Убираем кавычки и пробелы
    let provider = provider.trim_matches('"').trim();

    // Преобразуем строку в Provider enum
    let provider: Tabler = provider.parse()?;

    // Вызываем as_ref() и возвращаем SVG как строку
    Ok(tera::Value::String(provider.as_str().to_string()))
}

/// Return true name of icon
/// ```tera
///  {{ get_icon_name(icon=social.icon) }}
/// ```
fn get_icon_name(
    args: &std::collections::HashMap<String, tera::Value>
) -> tera::Result<tera::Value> {
    // Получаем значение provider из аргументов
    let provider = match args.get("icon") {
        Some(val) => val.to_string(),
        None => {
            return Err(tera::Error::msg(
                "Icon argument is required. Check your function invocation in template",
            ));
        }
    };

    // Убираем кавычки и пробелы
    let provider = provider.trim_matches('"').trim();

    // Преобразуем строку в Provider enum
    let provider: Tabler = provider.parse()?;

    // Вызываем as_ref() и возвращаем SVG как строку
    Ok(tera::Value::String(provider.as_str_merget().to_string()))
}
