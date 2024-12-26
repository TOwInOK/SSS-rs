/// A renderer that outputs HTML with Tailwind CSS styling.
/// Uses Tailwind classes to apply consistent styling to HTML elements.
use card::component::{frame::Direction, text::Font, Component};

use crate::{
    format::{tailwindcss::TailwindFormatter, StyleFormatter},
    theme::Shading,
};

use super::ComponentLayout;

pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Wraps content in an HTML tag with a class.
    ///
    /// # Arguments
    /// * `tag` - The HTML tag name to use (e.g. "div", "p", "span")
    /// * `class` - CSS classes to apply to the tag
    /// * `content` - Inner HTML content to wrap inside the tag
    ///
    /// # Returns
    /// A String containing the wrapped HTML
    fn wrap_tag(tag: &str, class: &str, content: &str) -> String {
        if content.is_empty() {
            format!("<{} class=\"{}\"/>\n", tag, class)
        } else {
            format!("<{} class=\"{}\">\n{}\n</{}>", tag, class, content, tag)
        }
    }
}

impl ComponentLayout for HtmlRenderer {
    type Output = String;
    type Formatter = TailwindFormatter;

    /// Renders a component to HTML with Tailwind styling.
    ///
    /// # Arguments
    /// * `formatter` - Formatter that provides Tailwind CSS classes
    /// * `theme` - Theme colors and styling to apply
    /// * `component` - Component to render to HTML
    ///
    /// # Returns
    /// HTML string representation of the component
    fn render(
        formatter: &Self::Formatter,
        theme: &impl Shading,
        component: &Component,
    ) -> Self::Output {
        match component {
            Component::Text(text) => {
                let class = match text.font {
                    Font::Label => formatter.label(theme),
                    Font::SubLabel => formatter.sub_label(theme),
                    Font::Text => formatter.text(theme),
                    Font::Minor => formatter.text_minor(theme),
                };
                HtmlRenderer::wrap_tag("p", &class, text.text.as_ref())
            }

            Component::Link(link) => format!(
                "<a href=\"{}\" class=\"{}\">\n{}{}\n</a>\n",
                link.href,
                formatter.link(theme),
                link.text
                    .as_ref()
                    .map(|x| Self::render(formatter, theme, x))
                    .unwrap_or_default(),
                link.icon
                    .as_ref()
                    .map(|x| Self::render(formatter, theme, x))
                    .unwrap_or_default()
            ),

            Component::Field(field) => format!(
                "<input type=\"text\" value=\"{}\" class=\"{}\"/>\n",
                Self::render(formatter, theme, field.title.as_ref()),
                formatter.field(theme),
            ),

            Component::Icon(icon) => {
                HtmlRenderer::wrap_tag("i", &formatter.icon(theme), icon.as_str())
            }

            Component::Frame(frame) => {
                let class = match frame.direction {
                    Direction::Vertical => formatter.vertical_frame(theme),
                    Direction::Horizontal => formatter.horizontal_frame(theme),
                    Direction::ReversVertical => formatter.reversed_vertical_frame(theme),
                    Direction::ReversHorizontal => formatter.reversed_horizontal_frame(theme),
                };
                let content: String = frame
                    .data
                    .iter()
                    .map(|component| Self::render(formatter, theme, component))
                    .collect();
                HtmlRenderer::wrap_tag("div", &class, &content)
            }
        }
    }

    /// Wraps the rendered component in a complete HTML document.
    ///
    /// # Arguments
    /// * `formatter` - Formatter that provides Tailwind CSS classes
    /// * `theme` - Theme colors and styling to apply
    /// * `component` - Component HTML to wrap in document
    ///
    /// # Returns
    /// Complete HTML document as a string
    fn finallyse(
        formatter: &Self::Formatter,
        theme: &impl Shading,
        component: Self::Output,
    ) -> Self::Output {
        format!(
            "<!DOCTYPE html>\n\
            <html lang=\"en\">\n\
            <head>\n\
                <meta charset=\"UTF-8\">\n\
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
                <script src=\"https://cdn.tailwindcss.com\"></script>\n\
            </head>\n\
            <body class=\"{}\">\n\
                {}\n\
            </body>\n\
            </html>",
            formatter.body(theme),
            component
        )
    }
}
