//! Rendering engine for user profile layouts and components
//!
//! This module provides a complete system for generating and rendering user profile layouts.
//! It supports multiple output formats including HTML with Tailwind CSS, standalone HTML+CSS,
//! and Leptos components. The rendering pipeline includes layout generation, formatting,
//! and theme application through the Shading trait interface.
use components::prelude::*;
use sss_core::{RenderTypeProvider, UserProvider};

use crate::{component_layout::ComponentLayout, theme::Shading};

/// Trait defining the layout generation and rendering interface for user profiles
///
/// Provides methods to generate different sections of a user profile layout
/// and render them in various output formats
pub trait Layout<C, U, S>
where
    C: ComponentLayout<S>,
    U: UserProvider,
    S: Shading,
{
    /// Generates the overall layout structure
    fn layout<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;

    /// Generates the user information section
    fn user<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;

    /// Generates a single specialization section
    fn specialization<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;

    /// Generates the specializations container section
    fn specializations<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;

    /// Generates the about section
    fn about<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;

    /// Generates the skills container section
    fn skills<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;

    /// Generates a single skill item section
    fn skill<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;

    /// Generates the social links container section
    fn socials<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;

    /// Generates a single social link item section
    fn social<'a>(component_layout: &C, settings: &'a U) -> Component<'a>;
}

pub trait Render<C, U, S, L, D, R, O>
where
    C: ComponentLayout<S>,
    U: UserProvider,
    S: Shading,
    L: Layout<C, U, S>,
    D: UserProvider,
    R: RenderTypeProvider,
{
    /// Transforms the profile layout into the requested output format
    fn render_layout(data: &D, render_type: &R, layout: &L, shade: &S) -> O;
    /// Applies final processing to the rendered output
    ///
    /// Performs format-specific post-processing and cleanup on the rendered component
    /// based on the output format type and theme settings
    fn render_layout_finaly(component: O, shade: &S) -> O;
}
