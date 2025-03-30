use std::borrow::Cow;

use render::{
    Component,
    layout::{Layout, Limitations},
    render::Render,
};
use tera::{Context, Tera};
use theme::Shade;

use crate::tools::gen_css;

use super::{html_meta::Meta, tools::TeraData};

/// Wrapper for Tera component [HtmlTeraRender]
/// It's wrap component into Html document
#[derive(Debug, Clone)]
pub struct HtmlTeraFinalize<'l, 'c, C, L>
where
    C: Component<String> + TeraData + Clone,
    L: Layout<String> + Clone,
    Self: Component<String> + TeraData,
{
    /// Finalize template
    pub meta: Meta,
    pub layout: Cow<'l, L>,
    pub component: Cow<'c, C>,
}

impl<C, L> Render<String> for HtmlTeraFinalize<'_, '_, C, L>
where
    C: Component<String> + TeraData + Clone,
    L: Layout<String> + Clone,
    Self: TeraData + Layout,
{
    fn render(&self) -> render::Result<Cow<String>> {
        // render card
        let card = self.component.render()?;
        // render css
        let css = gen_css(&self.get_theme().get_encre_css_config(), &card);
        // Init tera
        let mut tera = Tera::default();
        // Add template
        tera.add_raw_template("template", self.get_layout().as_str())?;
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

        let rendered = tera.render("template", &context)?;
        Ok(Cow::Owned(rendered))
    }
}

impl<C, L> Layout for HtmlTeraFinalize<'_, '_, C, L>
where
    C: Component<String> + TeraData + Clone,
    L: Layout<String> + Clone,
{
    fn template(&self) -> Cow<String> {
        self.layout.template()
    }
}

impl<C, L> Component<String> for HtmlTeraFinalize<'_, '_, C, L>
where
    C: Component<String> + TeraData + Clone,
    L: Layout<String> + Clone,
{
}

impl<C, L> Limitations for HtmlTeraFinalize<'_, '_, C, L>
where
    C: Component<String> + TeraData + Clone,
    L: Layout<String> + Clone,
{
    fn limitations(&self) -> Option<Cow<sss_core::LayoutLimitations>> {
        self.layout.limitations()
    }
}
