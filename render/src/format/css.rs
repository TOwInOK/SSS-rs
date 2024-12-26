use crate::theme::Shading;

use super::StyleFormatter;

/// CssFormatter implements style formatting for CSS output
pub struct CssFormatter;

impl StyleFormatter for CssFormatter {
    /// Generates CSS style for body background
    fn body(&self, theme: &impl Shading) -> String {
        format!("background-color: {};", theme.get_colors().thirdly)
    }

    /// Generates CSS style for primary labels
    fn label(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; font-weight: 600; font-size: 1.125rem; line-height: 1.75rem;",
            theme.get_colors().primary
        )
    }

    /// Generates CSS style for secondary labels
    fn sub_label(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; font-weight: 500; font-size: 1rem; line-height: 1.5rem;",
            theme.get_colors().secondary
        )
    }

    /// Generates CSS style for regular text
    fn text(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; font-size: 1rem; line-height: 1.5rem;",
            theme.get_colors().primary
        )
    }

    /// Generates CSS style for minor text elements
    fn text_minor(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; font-size: 0.875rem; line-height: 1.25rem;",
            theme.get_colors().thirdly
        )
    }

    /// Generates CSS style for horizontal frame containers
    fn horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
            "display: flex; flex-direction: row; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            theme.get_paddings().border,
            theme.get_colors().border,
            theme.get_paddings().frame,
            theme.get_gaps().frame
        )
    }

    /// Generates CSS style for reversed horizontal frame containers
    fn reversed_horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
            "display: flex; flex-direction: row-reverse; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            theme.get_paddings().border,
            theme.get_colors().border,
            theme.get_paddings().frame,
            theme.get_gaps().frame
        )
    }

    /// Generates CSS style for vertical frame containers
    fn vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
            "display: flex; flex-direction: column; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            theme.get_paddings().border,
            theme.get_colors().border,
            theme.get_paddings().frame,
            theme.get_gaps().frame
        )
    }

    /// Generates CSS style for reversed vertical frame containers
    fn reversed_vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
            "display: flex; flex-direction: column-reverse; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            theme.get_paddings().border,
            theme.get_colors().border,
            theme.get_paddings().frame,
            theme.get_gaps().frame
        )
    }

    /// Generates CSS style for hyperlinks
    fn link(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; text-decoration: none; cursor: pointer;",
            theme.get_colors().primary
        )
    }

    /// Generates CSS style for form input fields
    fn field(&self, theme: &impl Shading) -> String {
        format!(
            "border: {}px solid {}; border-radius: 0.25rem; padding: {}px; outline: none;",
            theme.get_paddings().border,
            theme.get_colors().border,
            theme.get_paddings().button
        )
    }

    /// Generates CSS style for icons
    fn icon(&self, theme: &impl Shading) -> String {
        format!(
            "width: 1.5rem; height: 1.5rem; color: {};",
            theme.get_colors().primary
        )
    }
}
