use std::error::Error;

use components::prelude::*;
use render::prelude::*;
use tera::{Context, Tera};
/// Tera templater
pub struct UmbrellaHtmlTeraRender;
impl Layout<Result<String, Box<dyn Error>>, Sections, Theme> for UmbrellaHtmlTeraRender {
    fn render(
        &self,
        component: Sections,
        shade: &Theme,
    ) -> Result<String, Box<dyn Error>> {
        let tera = Tera::new("src/layouts/umbrella/tera/*.html.tera")?;
        let c: Vec<String> = tera.get_template_names().map(|x| x.to_string()).collect();
        let mut context = Context::new();
        context.insert("shade", shade);
        context.insert("user", &component.user_info());
        context.insert("specs", &component.specifications());
        context.insert("socials", &component.socials());
        context.insert("skills", &component.skills());
        context.insert("about", &component.about());

        let rendered = tera.render("layout.html.tera", &context)?;
        Ok(rendered)
    }
}
