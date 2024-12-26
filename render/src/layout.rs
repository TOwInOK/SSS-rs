//! Rendering engine for user profile layouts and components
//!
//! This module provides a complete system for generating and rendering user profile layouts.
//! It supports multiple output formats including HTML with Tailwind CSS, standalone HTML+CSS,
//! and Leptos components. The rendering pipeline includes layout generation, formatting,
//! and theme application through the Shading trait interface.

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
    format::{css::CssFormatter, tailwindcss::TailwindFormatter},
    generate::{html::HtmlRenderer, html_css::HtmlCssRenderer, leptos::LeptosRenderer, Renderer},
    theme::Shading,
};

/// Creates a component tree representing the user profile layout
///
/// Generates a hierarchical component structure from the provided user settings.
/// The layout includes sections for:
/// - Name and nickname
/// - Professional specializations
/// - About/bio section
/// - Main skills and expertise
/// - Social media links and profiles
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

/// Type representing the various supported output formats
///
/// Each variant contains the rendered profile in a specific format:
/// - HTML: Basic HTML with Tailwind classes
/// - HTMLCSS: Standalone HTML with embedded CSS styles
/// - LEPTOS: Leptos framework view components
pub enum RenderOut {
    /// Plain HTML output
    HTML(String),
    /// HTML with CSS styles output
    HTMLCSS(String),
    /// Leptos view component output
    LEPTOS(AnyView),
}

/// Transforms the profile layout into the requested output format
///
/// Takes the user settings and theme configuration to render the profile.
/// The output format is determined by the render_type setting and can be:
/// - HTML with Tailwind classes
/// - HTML with embedded CSS
/// - Leptos view components
pub fn render(settings: &Settings, theme: &impl Shading) -> RenderOut {
    let component = layout(settings);
    match settings.render_type {
        sss_core::types::render::Render::HTML => {
            let formatter = TailwindFormatter;
            RenderOut::HTML(HtmlRenderer::render(&formatter, theme, &component))
        }
        sss_core::types::render::Render::LEPTOS => {
            let formatter = TailwindFormatter;
            RenderOut::LEPTOS(LeptosRenderer::render(&formatter, theme, &component))
        }
        sss_core::types::render::Render::HTMLCSS => {
            let formatter = CssFormatter;
            RenderOut::HTMLCSS(HtmlCssRenderer::render(&formatter, theme, &component))
        }
    }
}

/// Applies final processing to the rendered output
pub fn finallyse(settings: &Settings, theme: &impl Shading) -> RenderOut {
    let render = render(settings, theme);
    match render {
        RenderOut::HTML(e) => {
            let formatter = TailwindFormatter;
            RenderOut::HTML(HtmlRenderer::finallyse(&formatter, theme, e))
        }
        RenderOut::LEPTOS(e) => {
            let formatter = TailwindFormatter;
            RenderOut::LEPTOS(LeptosRenderer::finallyse(&formatter, theme, e))
        }
        RenderOut::HTMLCSS(e) => {
            let formatter = CssFormatter;
            RenderOut::HTMLCSS(HtmlCssRenderer::finallyse(&formatter, theme, e))
        }
    }
}
