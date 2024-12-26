use crate::theme::Shading;
pub mod css;
pub mod tailwindcss;

/// Trait for formatting styles in different CSS frameworks
pub trait StyleFormatter {
    /// Returns base body styles
    fn body(&self, theme: &impl Shading) -> String;
    /// Returns label styles
    fn label(&self, theme: &impl Shading) -> String;
    /// Returns sub-label styles
    fn sub_label(&self, theme: &impl Shading) -> String;
    /// Returns text styles
    fn text(&self, theme: &impl Shading) -> String;
    /// Returns minor text styles
    fn text_minor(&self, theme: &impl Shading) -> String;

    /// Returns horizontal frame styles
    fn horizontal_frame(&self, theme: &impl Shading) -> String;
    /// Returns reversed horizontal frame styles
    fn reversed_horizontal_frame(&self, theme: &impl Shading) -> String;
    /// Returns vertical frame styles
    fn vertical_frame(&self, theme: &impl Shading) -> String;
    /// Returns reversed vertical frame styles
    fn reversed_vertical_frame(&self, theme: &impl Shading) -> String;

    /// Returns link styles
    fn link(&self, theme: &impl Shading) -> String;
    /// Returns form field styles
    fn field(&self, theme: &impl Shading) -> String;
    /// Returns icon styles
    fn icon(&self, theme: &impl Shading) -> String;
}
