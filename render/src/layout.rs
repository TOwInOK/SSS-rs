use card::{
    component::{
        frame::{Direction, Frame},
        icon::Icon,
        link::Link,
        text::{Font, Text},
        Component,
    },
    frame, text,
};
use leptos::prelude::AnyView;
use sss_core::Settings;

use crate::{
    generate::{html::HtmlRenderer, html_css::HtmlCssRenderer, leptos::LeptosRenderer, Renderer},
    theme::{CssShading, TailwindShading},
};

pub fn layout(
    Settings {
        user,
        render_type: _,
    }: &Settings,
) -> Component<'_> {
    // Создаем основной вертикальный фрейм
    Component::Frame(Frame::new(
        vec![
            // Секция с именем и никнеймом
            frame!(Horizontal;
                vec![
                    text!(&user.name, Label),
                    text!(" (", Minor),
                    text!(&user.nickname.word),
                    text!(")", Minor),
                ]
            ),
            // Специализации
            frame!(Horizontal;
                vec![
                    text!("Специализации: ", SubLabel),
                    frame!(Horizontal;
                        user.specifications
                            .iter()
                            .map(|spec| text!(spec))
                            .collect()
                    )
                ]
            ),
            // О пользователе
            frame!(Vertical;
                vec![
                    text!("Обо мне:", SubLabel),
                    text!(&user.about.text)
                ]
            ),
            // Основные навыки
            frame!(Vertical;
                vec![
                    text!("Навыки:", SubLabel),
                    frame!(Horizontal;
                        user.skills
                            .iter()
                            .filter(|skill| skill.main)
                            .map(|skill| text!(&skill.skill))
                            .collect()
                    )
                ]
            ),
            // Социальные сети
            frame!(Vertical;
                vec![
                    text!("Социальные сети:", SubLabel),
                    frame!(Horizontal;
                        user.social_media
                            .iter()
                            .map(|social| {
                                let text_component = text!(&social.provider);
                                let icon_component = social.logo
                                    .as_ref()
                                    .map(|logo| Component::Icon(Icon::Custom(logo)));

                                Component::Link(Link::new(
                                    Some(text_component),
                                    &social.link,
                                    icon_component
                                ))
                            })
                            .collect()
                    )
                ]
            ),
        ],
        Direction::Vertical,
    ))
}

pub enum RenderOut {
    HTML(String),
    HTMLCSS(String),
    LEPTOS(AnyView),
}

pub fn render<T: TailwindShading + CssShading>(settings: &Settings, theme: &T) -> RenderOut {
    let component = layout(settings);
    match settings.render_type {
        sss_core::types::render::Render::HTML => {
            RenderOut::HTML(HtmlRenderer::render(theme, &component))
        }
        sss_core::types::render::Render::LEPTOS => {
            RenderOut::LEPTOS(LeptosRenderer::render(theme, &component))
        }
        sss_core::types::render::Render::HTMLCSS => {
            RenderOut::HTMLCSS(HtmlCssRenderer::render(theme, &component))
        }
    }
}

pub fn finallyse<T: TailwindShading + CssShading>(settings: &Settings, theme: &T) -> RenderOut {
    let render = render(settings, theme);
    match render {
        RenderOut::HTML(e) => RenderOut::HTML(HtmlRenderer::finallyse(theme, e)),
        RenderOut::LEPTOS(e) => RenderOut::LEPTOS(LeptosRenderer::finallyse(theme, e)),
        RenderOut::HTMLCSS(e) => RenderOut::HTMLCSS(HtmlCssRenderer::finallyse(theme, e)),
    }
}
