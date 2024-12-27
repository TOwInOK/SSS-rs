use crate::{theme::Shading, theming::Theming};
use components::prelude::Component;

/// A trait for rendering components to different output formats.
pub trait ComponentLayout<S>
where
    S: Shading,
{
    /// The output type produced by this renderer.
    type Output;
    /// The style formatter type used by this renderer.
    type Formatter: Theming<S>;

    /// Renders a component to the output format using the provided formatter and theme.
    fn render(formatter: &Self::Formatter, theme: S, component: &Component) -> Self::Output;

    /// Performs final processing on rendered component output.
    fn finallyse(formatter: &Self::Formatter, theme: S, component: Self::Output) -> Self::Output;
}
