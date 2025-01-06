use std::{borrow::Cow, error::Error};

use components::prelude::*;
use encre_css::Preflight;
use render::prelude::*;
use tera::{Context, Tera};

use crate::{css_inject::css_inject, gen_css::gen_css};
/// Tera templater
pub struct UmbrellaHtmlTeraRender;
impl Layout<Result<String, Box<dyn Error>>, Sections, Theme> for UmbrellaHtmlTeraRender {
    fn render(
        &self,
        component: Sections,
        shade: &Theme,
    ) -> Result<String, Box<dyn Error>> {
        let tera = Tera::new("src/layouts/umbrella/tera/*.html.tera")?;
        let mut context = Context::new();
        context.insert("shade", shade);
        context.insert("user", &component.user_info());
        context.insert("specs", &component.specifications());
        context.insert("socials", &component.socials());
        context.insert("skills", &component.skills());
        context.insert("about", &component.about());
        context.insert("font", shade.gfont_mono.0);

        let rendered = tera.render("card.html.tera", &context)?;
        Ok(rendered)
    }

    fn finalize(
        &self,
        rendered: Result<String, Box<dyn Error>>,
        shade: &Theme,
    ) -> Result<String, Box<dyn Error>> {
        let content = rendered?;

        let mut config = encre_css::Config::default();
        config.preflight = Preflight::Full {
            ring_color: None,
            border_color: None,
            placeholder_color: None,
            font_family_sans: Some(Cow::Borrowed(shade.gfont_regular.0)),
            font_family_mono: Some(Cow::Borrowed(shade.gfont_mono.0)),
        };

        let css = gen_css(Some(config), &content);

        let content = css_inject(content, css);

        let tera = Tera::new("src/layouts/umbrella/tera/*.html.tera")?;
        let mut context = Context::new();
        context.insert("content", &content);
        context.insert("font_regular", &shade.gfont_regular.1);
        context.insert("font_mono", &shade.gfont_mono.1);

        let rendered = tera.render("card_final.html.tera", &context)?;

        Ok(rendered)
    }
}
