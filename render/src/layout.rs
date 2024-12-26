//! Rendering engine for user profile layouts and components
//!
//! This module provides a complete system for generating and rendering user profile layouts.
//! It supports multiple output formats including HTML with Tailwind CSS, standalone HTML+CSS,
//! and Leptos components. The rendering pipeline includes layout generation, formatting,
//! and theme application through the Shading trait interface.
pub mod umbrella;

use card::prelude::*;
use leptos::prelude::AnyView;
use sss_core::{RenderTypeProvider, UserProvider};

use crate::{
    component_layout::{
        html::HtmlRenderer, html_css::HtmlCssRenderer, leptos::LeptosRenderer, ComponentLayout,
    },
    format::{css::CssFormatter, tailwindcss::TailwindFormatter},
    theme::Shading,
};

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

/// Trait defining the layout generation and rendering interface for user profiles
///
/// Provides methods to generate different sections of a user profile layout
/// and render them in various output formats
pub trait Layout {
    /// Generates the overall layout structure
    fn layout<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Generates the user information section
    fn user<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Generates a single specialization section
    fn specialization<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Generates the specializations container section
    fn specializations<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Generates the about section
    fn about<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Generates the skills container section
    fn skills<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Generates a single skill item section
    fn skill<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Generates the social links container section
    fn socials<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Generates a single social link item section
    fn social<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a>;

    /// Transforms the profile layout into the requested output format
    ///
    /// Takes the user settings and theme configuration to render the profile.
    /// The output format is determined by the render_type setting and can be:
    /// - HTML with Tailwind classes
    /// - HTML with embedded CSS
    /// - Leptos view components
    fn render(
        &self,
        user: &impl UserProvider,
        component_layout: &impl ComponentLayout,
        render_type: &impl RenderTypeProvider,
        theme: &impl Shading,
    ) -> RenderOut {
        let component = self.layout(component_layout, user);
        match render_type.render_type() {
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
    ///
    /// Performs format-specific post-processing and cleanup on the rendered component
    /// based on the output format type and theme settings
    fn render_finaly(&self, component: RenderOut, theme: &impl Shading) -> RenderOut {
        match component {
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
}
