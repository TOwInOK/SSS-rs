use std::error::Error;

use render::{layout::GetData, prelude::*};
use sss_core::{types::provider::Provider, Settings};
use tera::{Context, Tera};

const UMBRELLA_TERA_CARD_TEMPLATE: &str = include_str!("tera/card.html.tera");
const UMBRELLA_TERA_TEMPLATE: &str = include_str!("tera/card_final.html.tera");

use crate::tools::gen_css;

/// Tera templater
#[derive(Clone, Debug)]
pub struct UmbrellaHtmlTeraRender<'a, 'b> {
    pub settings: &'a Settings,
    pub theme: &'b Theme,
}
impl<'a, 'b> UmbrellaHtmlTeraRender<'a, 'b> {
    pub fn new(
        settings: &'a Settings,
        theme: &'b Theme,
    ) -> Self {
        Self {
            settings,
            theme,
        }
    }
}

impl<'a, 'b> Layout<'a, 'b> for UmbrellaHtmlTeraRender<'a, 'b> {
    fn render(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let theme = self.get_theme();
        let data = self.get_data();
        let mut tera = Tera::default();
        tera.add_raw_template("card.html", UMBRELLA_TERA_CARD_TEMPLATE)?;

        tera.register_function("get_svg", get_svg);
        let mut context = Context::new();

        context.insert("theme", &theme.colors);

        context.insert("user", &data.user);
        context.insert("about", &data.about);

        context.insert("specs", &data.specifications);
        context.insert("socials", &data.socials);
        context.insert("skills", &data.skills);

        let rendered = tera.render("card.html", &context)?;
        Ok(rendered)
    }
}

impl<'a, 'b> Finalize<'a, 'b> for UmbrellaHtmlTeraRender<'a, 'b> {
    fn finalize(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let rendered = self.render();
        let encre_css_config = Self::get_encre_css_config();
        let content = rendered?;

        let css = gen_css(&encre_css_config, &content);

        let mut tera = Tera::default();
        tera.add_raw_template("layout.html", UMBRELLA_TERA_TEMPLATE)?;
        let mut context = Context::new();
        context.insert("content", &content);
        context.insert("style", &css);
        context.insert("font_regular", Self::regular_font().1);
        context.insert("font_mono", Self::mono_font().1);

        let rendered = tera.render("layout.html", &context)?;

        Ok(rendered)
    }
}

impl<'a, 'b> GetData<'a, 'b> for UmbrellaHtmlTeraRender<'a, 'b> {
    fn get_data(&self) -> &Settings {
        self.settings
    }

    fn get_theme(&self) -> &Theme {
        self.theme
    }

    fn regular_font() -> (&'static str, &'static str) {
        (
            "PT Mono",
            "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
        )
    }

    fn mono_font() -> (&'static str, &'static str) {
        (
            "PT Mono",
            "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
        )
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
