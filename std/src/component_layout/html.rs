use super::ComponentLayout;
use crate::{
    format::{umberella::TailwindFormatter, StyleFormatter},
    theme::Shading,
};
use card::component::{frame::Direction, text::Font, Component};

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

impl ComponentLayout for HtmlRenderer {
    type Output = String;
    type Formatter = TailwindFormatter;

    fn render(
        formatter: &Self::Formatter,
        theme: &impl Shading,
        component: &Component,
    ) -> Self::Output {
        let rendered = match component {
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
                let content = format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"{}\" {}/></svg>",
                    formatter.icon(theme),
                    icon.as_ref()
                );
                HtmlRenderer::wrap_tag("i", &formatter.icon(theme), content.as_str())
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
        };

        // Оборачиваем в внешний frame
        HtmlRenderer::wrap_tag("div", &formatter.frame(theme), &rendered)
    }
    // Добавь в theme данные шрифтов (cdn), скриптов, метатегов
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
                <script>\n\
                    tailwind.config = {{\n\
                        theme: {{\n\
                            extend: {{\n\
                                fontFamily: {{\n\
                                    mono: ['PT Mono', 'monospace'],\n\
                                }},\n\
                            }},\n\
                        }},\n\
                    }};\n\
                </script>\n\
                <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n\
                <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n\
                <link href=\"https://fonts.googleapis.com/css2?family=PT+Mono&display=swap\" rel=\"stylesheet\">\n\
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
