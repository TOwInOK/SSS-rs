use crate::theme::Shading;

/// Trait for formatting styles in different CSS frameworks.
///
/// This trait provides methods for generating framework-specific CSS styles
/// for various UI components like text, frames, links etc. Each method takes
/// a theme parameter implementing the Shading trait to apply consistent styling.
pub trait Theming<S>
where
    S: Shading,
{
    /// Returns base body styles for the main content area
    fn body(&self, theme: S) -> String;
    /// Returns label styles for form labels and headings
    fn label(&self, theme: S) -> String;
    /// Returns sub-label styles for secondary/supporting text under labels
    fn sub_label(&self, theme: S) -> String;
    /// Returns text styles for regular body text content
    fn text(&self, theme: S) -> String;
    /// Returns minor text styles for less emphasized text content
    fn text_minor(&self, theme: S) -> String;

    /// Returns horizontal frame styles for laying out content in a row
    fn horizontal_frame(&self, theme: S) -> String;
    /// Returns reversed horizontal frame styles for right-to-left layouts
    fn reversed_horizontal_frame(&self, theme: S) -> String;
    /// Returns vertical frame styles for laying out content in a column
    fn vertical_frame(&self, theme: S) -> String;
    /// Returns reversed vertical frame styles for bottom-to-top layouts
    fn reversed_vertical_frame(&self, theme: S) -> String;

    /// Returns link styles for clickable text elements
    fn link(&self, theme: S) -> String;
    /// Returns form field styles for input elements
    fn field(&self, theme: S) -> String;
    /// Returns icon styles for decorative graphical elements
    fn icon(&self, theme: S) -> String;
    /// Return frame base element
    fn frame(&self, theme: S) -> String;
}
