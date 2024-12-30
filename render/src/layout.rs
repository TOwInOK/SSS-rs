//! Rendering engine for user profile layouts and components
//!
//! This module provides a complete system for generating and rendering user profile layouts.
//! It supports multiple output formats including HTML with Tailwind CSS, standalone HTML+CSS,
//! and Leptos components. The rendering pipeline includes layout generation, formatting,
//! and theme application through the Shading trait interface.
use sss_core::UserProvider;

use crate::{component_layout::ComponentLayout, theme::Shading};

pub trait Layout<Out, C, U, S, D>
where
    C: ComponentLayout<C, S, Out>,
    D: UserProvider + UserProvider,
    S: Shading,
{
    /// Transforms the profile layout into the requested output format
    fn render(component: C, shade: &S, data: &D) -> Out;
    /// Applies final processing to the rendered output
    ///
    /// Performs format-specific post-processing and cleanup on the rendered component
    /// based on the output format type and theme settings
    fn finallyse(component: Out, shade: &S) -> Out;
}
