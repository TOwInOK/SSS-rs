use card::component::{frame::Direction, text::Font, Component};

use crate::theme::{CssShading, TailwindShading};

use super::Renderer;

pub struct HtmlRenderer;

impl HtmlRenderer {
    fn wrap_tag(tag: &str, class: &str, content: &str) -> String {
        if content.is_empty() {
            format!("<{} class=\"{}\"/>\n", tag, class)
        } else {
            format!("<{} class=\"{}\">\n{}\n</{}>", tag, class, content, tag)
        }
    }
}

impl<T: TailwindShading + CssShading> Renderer<T> for HtmlRenderer {
    type Output = String;

    fn render(theme: &T, component: &Component) -> Self::Output {
        match component {
            Component::Text(text) => {
                let class = match text.font {
                    Font::Label => TailwindShading::label(theme),
                    Font::SubLabel => TailwindShading::sub_label(theme),
                    Font::Text => TailwindShading::text(theme),
                    Font::Minor => TailwindShading::text_minor(theme),
                };
                HtmlRenderer::wrap_tag("p", &class, text.text)
            }

            Component::Link(link) => format!(
                "<a href=\"{}\" class=\"{}\">\n{}{}\n</a>\n",
                link.href,
                TailwindShading::link(theme),
                link.text
                    .as_ref()
                    .map(|x| HtmlRenderer::render(theme, x))
                    .unwrap_or_default(),
                link.icon
                    .as_ref()
                    .map(|x| HtmlRenderer::render(theme, x))
                    .unwrap_or_default()
            ),

            Component::Field(field) => format!(
                "<input type=\"text\" value=\"{}\" class=\"{}\"/>\n",
                HtmlRenderer::render(theme, field.title),
                TailwindShading::field(theme),
            ),

            Component::Icon(icon) => {
                HtmlRenderer::wrap_tag("i", &TailwindShading::icon(theme), icon.as_str())
            }

            Component::Frame(frame) => {
                let class = match frame.direction {
                    Direction::Vertical => TailwindShading::vertical_frame(theme),
                    Direction::Horizontal => TailwindShading::horizontal_frame(theme),
                    Direction::ReversVertical => TailwindShading::reversed_vertical_frame(theme),
                    Direction::ReversHorizontal => {
                        TailwindShading::reversed_horizontal_frame(theme)
                    }
                };
                let content: String = frame
                    .data
                    .iter()
                    .map(|x| HtmlRenderer::render(theme, x))
                    .collect();
                HtmlRenderer::wrap_tag("div", &class, &content)
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
                <script src=\"https://cdn.tailwindcss.com\"></script>\n\
            </head>\n\
            <body class=\"{}\">\n\
                {}\n\
            </body>\n\
            </html>",
            TailwindShading::body(theme),
            component
        )
    }
}
