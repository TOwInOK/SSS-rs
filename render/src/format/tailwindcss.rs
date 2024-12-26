use crate::theme::Shading;

use super::StyleFormatter;

/// Formatter for generating Tailwind CSS classes based on a theme
pub struct TailwindFormatter;

impl StyleFormatter for TailwindFormatter {
    fn body(&self, theme: &impl Shading) -> String {
        format!(
            "bg-[{}] min-h-screen flex items-center justify-center",
            theme.get_colors().secondary
        )
    }

    fn label(&self, theme: &impl Shading) -> String {
        format!("text-[{}] font-bold text-xl", theme.get_colors().primary)
    }

    fn sub_label(&self, theme: &impl Shading) -> String {
        format!("text-[{}] opacity-80 text-lg", theme.get_colors().primary)
    }

    fn text(&self, theme: &impl Shading) -> String {
        format!("text-[{}] text-base", theme.get_colors().primary)
    }

    fn text_minor(&self, theme: &impl Shading) -> String {
        format!("text-[{}] text-sm opacity-60", theme.get_colors().primary)
    }

    fn horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
               "flex items-center justify-center border-[{}] border-[{}] rounded-md p-{} gap-{} bg-[{}] shadow-lg shadow-[{}]/20",
               theme.get_colors().border,
               theme.get_paddings().border,
               theme.get_paddings().frame,
               theme.get_gaps().frame,
               theme.get_colors().secondary,
               theme.get_colors().thirdly
           )
    }

    fn reversed_horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
               "flex flex-row-reverse items-center justify-center border-[{}] border-[{}] rounded-md p-{} gap-{} bg-[{}] shadow-lg shadow-[{}]/20",
               theme.get_colors().border,
               theme.get_paddings().border,
               theme.get_paddings().frame,
               theme.get_gaps().frame,
               theme.get_colors().secondary,
               theme.get_colors().thirdly
           )
    }

    fn vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
               "flex flex-col items-center justify-center border-[{}] border-[{}] rounded-md p-{} gap-{} bg-[{}] shadow-lg shadow-[{}]/20",
               theme.get_colors().border,
               theme.get_paddings().border,
               theme.get_paddings().frame,
               theme.get_gaps().frame,
               theme.get_colors().secondary,
               theme.get_colors().thirdly
           )
    }

    fn reversed_vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
               "flex flex-col-reverse items-center justify-center border-[{}] border-[{}] rounded-md p-{} gap-{} bg-[{}] shadow-lg shadow-[{}]/20",
               theme.get_colors().border,
               theme.get_paddings().border,
               theme.get_paddings().frame,
               theme.get_gaps().frame,
               theme.get_colors().secondary,
               theme.get_colors().thirdly
           )
    }

    fn link(&self, theme: &impl Shading) -> String {
        format!(
            "text-[{}] hover:underline cursor-pointer opacity-90",
            theme.get_colors().primary
        )
    }

    fn field(&self, theme: &impl Shading) -> String {
        format!(
            "border-[{}] border-[{}] rounded p-{} focus:outline-none focus:border-[{}] bg-[{}]/80",
            theme.get_colors().border,
            theme.get_paddings().border,
            theme.get_paddings().button,
            theme.get_colors().primary,
            theme.get_colors().secondary
        )
    }

    fn icon(&self, theme: &impl Shading) -> String {
        format!("w-6 h-6 text-[{}] opacity-80", theme.get_colors().primary)
    }
}
