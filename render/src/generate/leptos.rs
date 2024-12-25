use card::component::{frame::Direction, text::Font};
use html::{a, body, div, head, html, meta, p, script, ElementChild};
use leptos::*;
use prelude::{AnyView, ClassAttribute, CustomAttribute, IntoAny, RenderHtml};

use crate::theme::TailwindShading;

use super::Renderer;

pub struct LeptosRenderer;

impl LeptosRenderer {
    fn wrap_element(class: String, children: Vec<AnyView>) -> impl IntoView {
        div().class(class).child(children)
    }
}

impl<T: TailwindShading> Renderer<T> for LeptosRenderer {
    type Output = AnyView;
    fn render(theme: &T, component: &card::component::Component) -> Self::Output {
        match component {
            card::component::Component::Text(item) => {
                let class = match item.font {
                    Font::Label => theme.label(),
                    Font::SubLabel => theme.sub_label(),
                    Font::Text => theme.text(),
                    Font::Minor => theme.text_minor(),
                };
                p().class(class).child(item.text.to_html()).into_any()
            }
            card::component::Component::Frame(item) => {
                let class = match item.direction {
                    Direction::Vertical => theme.vertical_frame(),
                    Direction::Horizontal => theme.horizontal_frame(),
                    Direction::ReversVertical => theme.reversed_vertical_frame(),
                    Direction::ReversHorizontal => theme.reversed_horizontal_frame(),
                };
                let children = item
                    .data
                    .iter()
                    .map(|x| LeptosRenderer::render(theme, x).into_any())
                    .collect();
                Self::wrap_element(class, children).into_any()
            }
            card::component::Component::Link(item) => {
                let class = theme.link();
                match &item.text {
                    Some(e) => a()
                        .href(item.href.to_html())
                        .class(class)
                        .child(Self::render(theme, e))
                        .into_any(),
                    None => a().href(item.href.to_html()).class(class).into_any(),
                }
            }
            card::component::Component::Field(item) => {
                let class = theme.field();
                let label = Self::render(theme, item.title);
                let value = item
                    .element
                    .map(|x| Self::render(theme, x))
                    .unwrap_or(div().class("hidden").into_any());
                let icon = item
                    .icon
                    .map(|x| Self::render(theme, x))
                    .unwrap_or(div().class("hidden").into_any());
                div()
                    .class(class)
                    .child(vec![label, value, icon])
                    .into_any()
            }
            card::component::Component::Icon(item) => {
                let class = theme.icon();
                div().class(class).child(item.as_str().to_html()).into_any()
            }
        }
    }

    fn finallyse(_theme: &T, component: Self::Output) -> Self::Output {
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
