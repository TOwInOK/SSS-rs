use crate::theme::Shading;

/// A trait for rendering components to different output formats.
pub trait ComponentLayout<C, S: Shading, O> {
    /// The style formatter type used by this renderer.

    /// Renders a component to the output format using the provided formatter and theme.
    fn render(component: C, theme: &S) -> O;

    /// Performs final processing on rendered component output.
    fn finallyse(component: O, theme: S) -> O;
}
