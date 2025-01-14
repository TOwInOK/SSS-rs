use std::error::Error;

use render::{layout::GetSetData, prelude::*};
use sss_core::{types::provider::Provider, Settings};
use tera::{Context, Tera};

const UMBRELLA_TERA_CARD_TEMPLATE: &str = include_str!("tera/card.html.tera");
const UMBRELLA_TERA_TEMPLATE: &str = include_str!("tera/card_final.html.tera");

use crate::tools::{css_inject, gen_css};

/// Tera templater
#[derive(Clone, Debug)]
pub struct UmbrellaHtmlTeraRender<'a, 'b, 'c> {
    pub data: &'a Settings,
    pub theme: &'b Theme,
    pub encre_css: &'c encre_css::Config,
}

impl<'a, 'b, 'c> Layout<'a, 'b, 'c, Result<String, Box<dyn Error>>>
    for UmbrellaHtmlTeraRender<'a, 'b, 'c>
{
    fn render(&self) -> Result<String, Box<dyn Error>> {
        let theme = self.get_theme();
        let data = self.get_data();
        let mut tera = Tera::default();
        tera.add_raw_template("card.html", UMBRELLA_TERA_CARD_TEMPLATE)?;

        tera.register_function("get_svg", get_svg);
        let mut context = Context::new();

        context.insert("theme", &theme.colors);
        context.insert("font", theme.gfont_mono.0);

        context.insert("user", &data.user);
        context.insert("about", &data.about);

        context.insert("specs", &data.specifications);
        context.insert("socials", &data.socials);
        context.insert("skills", &data.skills);

        let rendered = tera.render("card.html", &context)?;
        Ok(rendered)
    }
}

impl<'a, 'b, 'c> Finalize<'a, 'b, 'c, Result<String, Box<dyn Error>>>
    for UmbrellaHtmlTeraRender<'a, 'b, 'c>
{
    fn finalize(&self) -> Result<String, Box<dyn Error>> {
        let rendered = self.render();
        let encre_css_config = self.get_encre_css_config();
        let theme = self.get_theme();
        let content = rendered?;

        let css = gen_css(encre_css_config, &content);
        let content = css_inject(content, css);

        let mut tera = Tera::default();
        tera.add_raw_template("layout.html", UMBRELLA_TERA_TEMPLATE)?;
        let mut context = Context::new();
        context.insert("content", &content);
        context.insert("font_regular", theme.gfont_regular.1);
        context.insert("font_mono", theme.gfont_mono.1);

        let rendered = tera.render("layout.html", &context)?;

        Ok(rendered)
    }
}

impl<'a, 'b, 'c> GetSetData<'a, 'b, 'c> for UmbrellaHtmlTeraRender<'a, 'b, 'c> {
    fn get_data(&self) -> &Settings {
        self.data
    }

    fn get_theme(&self) -> &Theme {
        self.theme
    }

    fn data(
        mut self,
        data: &'a Settings,
    ) -> Self {
        self.data = data;
        self
    }

    fn theme(
        mut self,
        theme: &'b Theme,
    ) -> Self {
        self.theme = theme;
        self
    }

    fn get_encre_css_config(&self) -> &encre_css::Config {
        self.encre_css
    }

    fn encre_css_config(
        mut self,
        config: &'c encre_css::Config,
    ) -> Self {
        self.encre_css = config;
        self
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
