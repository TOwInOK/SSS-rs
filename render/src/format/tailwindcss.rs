use crate::theme::Shading;

use super::StyleFormatter;
/// Formatter for generating Tailwind CSS classes based on a theme
pub struct TailwindFormatter;

impl StyleFormatter for TailwindFormatter {
    /// Returns Tailwind classes for styling the background of a body element
    fn body(&self, theme: &impl Shading) -> String {
        format!("bg-{}", theme.get_colors().thirdly)
    }

    /// Returns Tailwind classes for styling a primary label element
    fn label(&self, theme: &impl Shading) -> String {
        format!("text-{} font-semibold text-lg", theme.get_colors().primary)
    }

    /// Returns Tailwind classes for styling a secondary/sub label element
    fn sub_label(&self, theme: &impl Shading) -> String {
        format!(
            "text-{} font-medium text-base",
            theme.get_colors().secondary
        )
    }

    /// Returns Tailwind classes for styling regular text content
    fn text(&self, theme: &impl Shading) -> String {
        format!("text-{} text-base", theme.get_colors().primary)
    }

    /// Returns Tailwind classes for styling minor/secondary text content
    fn text_minor(&self, theme: &impl Shading) -> String {
        format!("text-{} text-sm", theme.get_colors().thirdly)
    }

    /// Returns Tailwind classes for a horizontal flex container with borders
    fn horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
            "flex flex-row border-{} border-{} rounded-md p-{} gap-{}",
            theme.get_colors().border,
            theme.get_paddings().border,
            theme.get_paddings().frame,
            theme.get_gaps().frame
        )
    }

    /// Returns Tailwind classes for a reversed horizontal flex container with borders
    fn reversed_horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
            "flex flex-row-reverse border-{} border-{} rounded-md p-{} gap-{}",
            theme.get_colors().border,
            theme.get_paddings().border,
            theme.get_paddings().frame,
            theme.get_gaps().frame
        )
    }

    /// Returns Tailwind classes for a vertical flex container with borders
    fn vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
            "flex flex-col border-{} border-{} rounded-md p-{} gap-{}",
            theme.get_colors().border,
            theme.get_paddings().border,
            theme.get_paddings().frame,
            theme.get_gaps().frame
        )
    }

    /// Returns Tailwind classes for a reversed vertical flex container with borders
    fn reversed_vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
            "flex flex-col-reverse border-{} border-{} rounded-md p-{} gap-{}",
            theme.get_colors().border,
            theme.get_paddings().border,
            theme.get_paddings().frame,
            theme.get_gaps().frame
        )
    }

    /// Returns Tailwind classes for styling a clickable link element
    fn link(&self, theme: &impl Shading) -> String {
        format!(
            "text-{} hover:underline cursor-pointer",
            theme.get_colors().primary
        )
    }

    /// Returns Tailwind classes for styling an input field
    fn field(&self, theme: &impl Shading) -> String {
        format!(
            "border-{} border-{} rounded p-{} focus:outline-none focus:border-{}",
            theme.get_colors().border,
            theme.get_paddings().border,
            theme.get_paddings().button,
            theme.get_colors().primary
        )
    }

    /// Returns Tailwind classes for styling an icon element
    fn icon(&self, theme: &impl Shading) -> String {
        format!("w-6 h-6 text-{}", theme.get_colors().primary)
    }
}
