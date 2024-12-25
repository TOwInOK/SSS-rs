use super::Renderer;
use crate::theme::CssShading;
use card::component::{frame::Direction, text::Font, Component};

pub struct HtmlCssRenderer;

impl HtmlCssRenderer {
    fn wrap_tag(tag: &str, class: &str, content: &str) -> String {
        if content.is_empty() {
            format!("<{} class=\"{}\"/>\n", tag, class)
        } else {
            format!("<{} class=\"{}\">\n    {}\n</{}>", tag, class, content, tag)
        }
    }

    fn generate_styles<T: CssShading>(theme: &T) -> String {
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
            theme.body(),
            theme.label(),
            theme.sub_label(),
            theme.text(),
            theme.text_minor(),
            theme.horizontal_frame(),
            theme.reversed_horizontal_frame(),
            theme.vertical_frame(),
            theme.reversed_vertical_frame(),
            theme.link(),
            theme.field(),
            theme.icon(),
        )
    }
}

impl<T: CssShading> Renderer<T> for HtmlCssRenderer {
    type Output = String;

    #[allow(clippy::only_used_in_recursion)]
    fn render(theme: &T, component: &Component) -> Self::Output {
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
                        .map(|x| Self::render(theme, x))
                        .unwrap_or_default(),
                    link.icon
                        .as_ref()
                        .map(|x| Self::render(theme, x))
                        .unwrap_or_default()
                );
                format!(
                    "<a href=\"{}\" class=\"sss-link\">\n    {}\n</a>\n",
                    link.href, inner_content
                )
            }

            Component::Field(field) => format!(
                "<input type=\"text\" value=\"{}\" class=\"sss-field\"/>\n",
                Self::render(theme, field.title),
            ),

            Component::Icon(icon) => Self::wrap_tag("i", "sss-icon", icon.as_str()),

            Component::Frame(frame) => {
                let class = match frame.direction {
                    Direction::Vertical => "sss-frame-vertical",
                    Direction::Horizontal => "sss-frame-horizontal",
                    Direction::ReversVertical => "sss-frame-vertical-reverse",
                    Direction::ReversHorizontal => "sss-frame-horizontal-reverse",
                };
                let content: String = frame.data.iter().map(|x| Self::render(theme, x)).collect();
                Self::wrap_tag("div", class, &content)
            }
        }
    }

    fn finallyse(theme: &T, component: Self::Output) -> Self::Output {
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
            Self::generate_styles(theme),
            component
        )
    }
}
