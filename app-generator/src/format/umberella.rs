use super::StyleFormatter;
use crate::theme::Shading;

pub struct TailwindFormatter;

impl StyleFormatter for TailwindFormatter {
    fn body(&self, theme: &impl Shading) -> String {
        format!(
            "min-h-screen bg-[{}] flex justify-center items-center p-4",
            theme.get_colors().secondary
        )
    }

    fn frame(&self, theme: &impl Shading) -> String {
        format!(
            "max-w-md w-full bg-black/20 rounded-2xl border border-[{}] p-1",
            theme.get_colors().thirdly
        )
    }

    fn label(&self, theme: &impl Shading) -> String {
        format!(
            "text-[{}] font-bold font-mono text-lg md:text-xl",
            theme.get_colors().primary
        )
    }

    fn sub_label(&self, theme: &impl Shading) -> String {
        format!(
            "text-[{}]/80 font-mono text-[10px] md:text-xs tracking-widest",
            theme.get_colors().thirdly
        )
    }

    fn text(&self, theme: &impl Shading) -> String {
        format!(
            "text-[{}] font-mono text-xs md:text-sm",
            theme.get_colors().primary
        )
    }

    fn text_minor(&self, theme: &impl Shading) -> String {
        format!(
            "text-[{}]/80 font-mono text-xs md:text-sm",
            theme.get_colors().border
        )
    }

    fn horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
            "p-4 rounded-lg border-t-2 border-b-2 border-[{}] flex items-center gap-4",
            theme.get_colors().thirdly
        )
    }

    fn reversed_horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
            "p-4 rounded-lg border-t-2 border-b-2 border-[{}] flex flex-row-reverse items-center gap-4",
            theme.get_colors().thirdly
        )
    }

    fn vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
            "p-4 rounded-lg border-t-2 border-b-2 border-[{}] flex flex-col gap-4",
            theme.get_colors().thirdly
        )
    }

    fn reversed_vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
            "p-4 rounded-lg border-t-2 border-b-2 border-[{}] flex flex-col-reverse gap-4",
            theme.get_colors().thirdly
        )
    }

    fn link(&self, theme: &impl Shading) -> String {
        format!(
            "text-[{}] font-mono text-xs md:text-sm hover:text-[{}]",
            theme.get_colors().primary,
            theme.get_colors().thirdly
        )
    }

    fn field(&self, theme: &impl Shading) -> String {
        format!(
            "font-mono text-xs md:text-sm text-[{}] border-[{}] rounded-lg p-2 focus:outline-none focus:border-[{}] bg-black/20",
            theme.get_colors().primary,
            theme.get_colors().border,
            theme.get_colors().thirdly
        )
    }

    fn icon(&self, theme: &impl Shading) -> String {
        format!("text-[{}] size-12 md:size-20", theme.get_colors().thirdly)
    }
}
