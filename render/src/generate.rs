pub mod html;
pub mod html_css;
pub mod leptos;
use crate::{format::StyleFormatter, theme::Shading};
use card::component::Component;

/// A trait for rendering components to different output formats.
pub trait Renderer {
    /// The output type produced by this renderer.
    type Output;
    /// The style formatter type used by this renderer.
    type Formatter: StyleFormatter;

    /// Renders a component to the output format using the provided formatter and theme.
    fn render(
        formatter: &Self::Formatter,
        theme: &impl Shading,
        component: &Component,
    ) -> Self::Output;

    /// Performs final processing on rendered component output.
    fn finallyse(
        formatter: &Self::Formatter,
        theme: &impl Shading,
        component: Self::Output,
    ) -> Self::Output;
}
