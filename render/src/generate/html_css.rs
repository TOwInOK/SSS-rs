use super::Renderer;
use crate::{
    format::{css::CssFormatter, StyleFormatter},
    theme::Shading,
};
use card::component::{frame::Direction, text::Font, Component};
/// HTML/CSS renderer that generates styled HTML output for components using pure CSS classes
pub struct HtmlCssRenderer;

impl HtmlCssRenderer {
    /// Wraps content in an HTML tag with a CSS class and proper indentation
    fn wrap_tag(tag: &str, class: &str, content: &str) -> String {
        if content.is_empty() {
            format!("<{} class=\"{}\"/>\n", tag, class)
        } else {
            format!("<{} class=\"{}\">\n    {}\n</{}>", tag, class, content, tag)
        }
    }

    /// Generates CSS styles for all component types based on the provided theme configuration
    ///
    /// Creates CSS classes for:
    /// - Body (.sss-body)
    /// - Text elements (.sss-label, .sss-sub-label, .sss-text, .sss-text-minor)
    /// - Frames (.sss-frame-*)
    /// - Links (.sss-link)
    /// - Fields (.sss-field)
    /// - Icons (.sss-icon)
    fn generate_styles(formatter: &CssFormatter, theme: &impl Shading) -> String {
        format!(
            "<style>\n\
                .sss-body {{\n{}  \n}}\n\
                .sss-label {{\n    {}\n}}\n\
                .sss-sub-label {{\n    {}\n}}\n\
                .sss-text {{\n    {}\n}}\n\
                .sss-text-minor {{\n    {}\n}}\n\
                .sss-frame-horizontal {{\n    {}\n}}\n\
                .sss-frame-horizontal-reverse {{\n    {}\n}}\n\
                .sss-frame-vertical {{\n    {}\n}}\n\
                .sss-frame-vertical-reverse {{\n    {}\n}}\n\
                .sss-link {{\n    {}\n}}\n\
                .sss-field {{\n    {}\n}}\n\
                .sss-icon {{\n    {}\n}}\n\
            </style>",
            formatter.body(theme),
            formatter.label(theme),
            formatter.sub_label(theme),
            formatter.text(theme),
            formatter.text_minor(theme),
            formatter.horizontal_frame(theme),
            formatter.reversed_horizontal_frame(theme),
            formatter.vertical_frame(theme),
            formatter.reversed_vertical_frame(theme),
            formatter.link(theme),
            formatter.field(theme),
            formatter.icon(theme),
        )
    }
}

impl Renderer for HtmlCssRenderer {
    type Output = String;
    type Formatter = CssFormatter;

    /// Renders a component to HTML markup with appropriate CSS styling
    ///
    /// Handles:
    /// - Text components with different font styles
    /// - Links with optional text and icons
    /// - Input fields
    /// - Icons
    /// - Frames with different layout directions
    #[allow(clippy::only_used_in_recursion)]
    fn render(
        formatter: &Self::Formatter,
        theme: &impl Shading,
        component: &Component,
    ) -> Self::Output {
        match component {
            Component::Text(text) => {
                let class = match text.font {
                    Font::Label => "sss-label",
                    Font::SubLabel => "sss-sub-label",
                    Font::Text => "sss-text",
                    Font::Minor => "sss-text-minor",
                };
                Self::wrap_tag("p", class, text.text)
            }

            Component::Link(link) => {
                let inner_content = format!(
                    "{}{}",
                    link.text
                        .as_ref()
                        .map(|x| Self::render(formatter, theme, x))
                        .unwrap_or_default(),
                    link.icon
                        .as_ref()
                        .map(|x| Self::render(formatter, theme, x))
                        .unwrap_or_default()
                );
                format!(
                    "<a href=\"{}\" class=\"sss-link\">\n    {}\n</a>\n",
                    link.href, inner_content
                )
            }

            Component::Field(field) => format!(
                "<input type=\"text\" value=\"{}\" class=\"sss-field\"/>\n",
                Self::render(formatter, theme, field.title),
            ),

            Component::Icon(icon) => Self::wrap_tag("i", "sss-icon", icon.as_str()),

            Component::Frame(frame) => {
                let class = match frame.direction {
                    Direction::Vertical => "sss-frame-vertical",
                    Direction::Horizontal => "sss-frame-horizontal",
                    Direction::ReversVertical => "sss-frame-vertical-reverse",
                    Direction::ReversHorizontal => "sss-frame-horizontal-reverse",
                };
                let content: String = frame
                    .data
                    .iter()
                    .map(|x| Self::render(formatter, theme, x))
                    .collect();
                Self::wrap_tag("div", class, &content)
            }
        }
    }

    /// Wraps the rendered component in a complete HTML5 document with styling
    ///
    /// Includes:
    /// - DOCTYPE declaration
    /// - Meta tags for charset and viewport
    /// - Generated CSS styles
    /// - Component content in a styled body
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
                    {}\n\
                </head>\n\
                <body class=\"sss-body\">\n    {}\n</body>\n\
                </html>",
            Self::generate_styles(formatter, theme),
            component
        )
    }
}
