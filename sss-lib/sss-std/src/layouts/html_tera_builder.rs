use std::error::Error;

use render::{layout::AdditionTeraData, prelude::*};
use sss_core::{types::provider::Provider, Settings};
use tera::{Context, Tera};

use crate::tools::gen_css;

/// Tera templater
#[derive(Clone, Debug)]
pub struct HtmlTeraRender<'a> {
    /// Settings
    pub settings: &'a Settings,
    /// Theme
    pub theme: &'static Theme,
    /// Card/Component template
    pub card_template: &'static str,
    /// Finalize template
    pub template: &'static str,
}
impl<'a> HtmlTeraRender<'a> {
    pub fn new(
        settings: &'a Settings,
        theme: &'static Theme,
        card_template: &'static str,
        template: &'static str,
    ) -> Self {
        Self {
            settings,
            theme,
            card_template,
            template,
        }
    }
}

impl Layout for HtmlTeraRender<'_> {
    fn render(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Init data
        let theme = self.get_theme();
        let settings = self.get_data();
        let mut tera = Tera::default();
        let layout = self.get_card_layout();

        // Add layout
        tera.add_raw_template("card.html", layout)?;
        // Register svg return fn
        // it need to use because tera don't use Display/ToString as expected for type
        tera.register_function("get_svg", get_svg);

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

        context.insert("primary", &theme.colors.primary);
        context.insert("secondary", &theme.colors.secondary);
        context.insert("thirdly", &theme.colors.thirdly);
        context.insert("border", &theme.colors.border);

        let rendered = tera.render("card.html", &context)?;
        Ok(rendered)
    }
}

impl Finalise for HtmlTeraRender<'_> {
    fn finalize(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        // render card
        let card = self.render()?;
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

        let rendered = tera.render("layout.html", &context)?;
        Ok(rendered)
    }
}

impl AdditionTeraData for HtmlTeraRender<'_> {
    #[inline]
    fn get_data(&self) -> &Settings {
        self.settings
    }

    #[inline]
    fn get_theme(&self) -> &Theme {
        self.theme
    }

    #[inline]
    fn get_card_layout(&self) -> &str {
        self.card_template
    }

    #[inline]
    fn get_layout(&self) -> &str {
        self.template
    }
}

fn get_svg(args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    // Получаем значение provider из аргументов
    let provider = match args.get("provider") {
        Some(val) => val.to_string(),
        None => return Err(tera::Error::msg("Provider argument is required")),
    };

    // Убираем кавычки и пробелы
    let provider = provider.trim_matches('"').trim();

    // Преобразуем строку в Provider enum
    let provider = match provider {
        "Github" => Provider::Github,
        "LinkedIn" => Provider::LinkedIn,
        "Telegram" => Provider::Telegram,
        _ => return Err(tera::Error::msg(format!("Invalid provider: {}", provider))),
    };

    // Вызываем as_ref() и возвращаем SVG как строку
    Ok(tera::Value::String(provider.as_ref().to_string()))
}
