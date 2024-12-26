/// A renderer implementation for Leptos web framework
///
/// This renderer converts card components into Leptos views styled with Tailwind CSS.
/// It handles rendering of various component types including:
/// - Text elements with different font styles
/// - Frames with vertical/horizontal layouts
/// - Links with optional text content
/// - Fields combining titles, values and icons
/// - Icon elements
///
/// The rendering process preserves the component hierarchy and applies theme-based
/// styles using the TailwindFormatter. The output can be finalized into a complete
/// HTML document with appropriate meta tags and Tailwind CSS setup.
use card::component::{frame::Direction, text::Font};
use html::{a, body, div, head, html, meta, p, script, ElementChild};
use leptos::*;
use prelude::{AnyView, ClassAttribute, CustomAttribute, IntoAny, RenderHtml};

use crate::{
    format::{tailwindcss::TailwindFormatter, StyleFormatter},
    theme::Shading,
};

use super::Renderer;

pub struct LeptosRenderer;

impl LeptosRenderer {
    /// Wraps child elements in a div with the specified class name
    ///
    /// # Arguments
    ///
    /// * `class` - CSS class name to apply to the wrapper div
    /// * `children` - Vector of child views to wrap
    ///
    /// # Returns
    ///
    /// Returns a new div element containing the children with the specified class
    fn wrap_element(class: String, children: Vec<AnyView>) -> impl IntoView {
        div().class(class).child(children)
    }
}

impl Renderer for LeptosRenderer {
    type Output = AnyView;
    type Formatter = TailwindFormatter;

    fn render(
        formatter: &Self::Formatter,
        theme: &impl Shading,
        component: &card::component::Component,
    ) -> Self::Output {
        match component {
            card::component::Component::Text(item) => {
                let class = match item.font {
                    Font::Label => formatter.label(theme),
                    Font::SubLabel => formatter.sub_label(theme),
                    Font::Text => formatter.text(theme),
                    Font::Minor => formatter.text_minor(theme),
                };
                p().class(class).child(item.text.to_html()).into_any()
            }
            card::component::Component::Frame(item) => {
                let class = match item.direction {
                    Direction::Vertical => formatter.vertical_frame(theme),
                    Direction::Horizontal => formatter.horizontal_frame(theme),
                    Direction::ReversVertical => formatter.reversed_vertical_frame(theme),
                    Direction::ReversHorizontal => formatter.reversed_horizontal_frame(theme),
                };
                let children = item
                    .data
                    .iter()
                    .map(|x| Self::render(formatter, theme, x))
                    .collect();
                Self::wrap_element(class, children).into_any()
            }
            card::component::Component::Link(item) => {
                let class = formatter.link(theme);
                match &item.text {
                    Some(e) => a()
                        .href(item.href.to_html())
                        .class(class)
                        .child(Self::render(formatter, theme, e))
                        .into_any(),
                    None => a().href(item.href.to_html()).class(class).into_any(),
                }
            }
            card::component::Component::Field(item) => {
                let class = formatter.field(theme);
                let title = Self::render(formatter, theme, item.title);
                let value = item
                    .element
                    .as_ref()
                    .map(|x| Self::render(formatter, theme, x))
                    .unwrap_or(div().class("hidden").into_any());
                let icon = item
                    .icon
                    .as_ref()
                    .map(|x| Self::render(formatter, theme, x))
                    .unwrap_or(div().class("hidden").into_any());
                div()
                    .class(class)
                    .child(vec![title, value, icon])
                    .into_any()
            }
            card::component::Component::Icon(item) => {
                let class = formatter.icon(theme);
                div().class(class).child(item.as_str().to_html()).into_any()
            }
        }
    }

    fn finallyse(
        _formatter: &Self::Formatter,
        _theme: &impl Shading,
        component: Self::Output,
    ) -> Self::Output {
        html()
            .child(
                head()
                    .child(meta().attr("charset", "UTF-8"))
                    .child(
                        meta()
                            .attr("name", "viewport")
                            .attr("content", "width=device-width, initial-scale=1.0"),
                    )
                    .child(script().attr("src", "https://cdn.tailwindcss.com")),
            )
            .child(body().child(component))
            .into_any()
    }
}
