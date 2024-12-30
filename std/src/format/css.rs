use super::StyleFormatter;
use crate::theme::Shading;

pub struct CssFormatter;

impl StyleFormatter for CssFormatter {
    fn body(&self, theme: &impl Shading) -> String {
        format!(
            "min-height: 100vh; background-color: {}; display: flex; justify-content: center; align-items: center; padding: 1rem;",
            theme.get_colors().secondary
        )
    }

    fn frame(&self, theme: &impl Shading) -> String {
        format!(
            "max-width: 28rem; width: 100%; background-color: rgba(0, 0, 0, 0.2); border-radius: 1rem; border: 1px solid {}; padding: 0.25rem;",
            theme.get_colors().thirdly
        )
    }

    fn label(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; font-weight: 700; font-family: monospace; font-size: 1.125rem; @media (min-width: 768px) {{ font-size: 1.25rem; }}",
            theme.get_colors().primary
        )
    }

    fn sub_label(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; opacity: 0.8; font-family: monospace; font-size: 0.625rem; letter-spacing: 0.1em; @media (min-width: 768px) {{ font-size: 0.75rem; }}",
            theme.get_colors().thirdly
        )
    }

    fn text(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; font-family: monospace; font-size: 0.75rem; @media (min-width: 768px) {{ font-size: 0.875rem; }}",
            theme.get_colors().primary
        )
    }

    fn text_minor(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; opacity: 0.8; font-family: monospace; font-size: 0.75rem; @media (min-width: 768px) {{ font-size: 0.875rem; }}",
            theme.get_colors().border
        )
    }

    fn horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
            "padding: 1rem; border-radius: 0.5rem; border-top: 2px solid {}; border-bottom: 2px solid {}; display: flex; align-items: center; gap: 1rem;",
            theme.get_colors().thirdly,
            theme.get_colors().thirdly
        )
    }

    fn reversed_horizontal_frame(&self, theme: &impl Shading) -> String {
        format!(
            "padding: 1rem; border-radius: 0.5rem; border-top: 2px solid {}; border-bottom: 2px solid {}; display: flex; flex-direction: row-reverse; align-items: center; gap: 1rem;",
            theme.get_colors().thirdly,
            theme.get_colors().thirdly
        )
    }

    fn vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
            "padding: 1rem; border-radius: 0.5rem; border-top: 2px solid {}; border-bottom: 2px solid {}; display: flex; flex-direction: column; gap: 1rem;",
            theme.get_colors().thirdly,
            theme.get_colors().thirdly
        )
    }

    fn reversed_vertical_frame(&self, theme: &impl Shading) -> String {
        format!(
            "padding: 1rem; border-radius: 0.5rem; border-top: 2px solid {}; border-bottom: 2px solid {}; display: flex; flex-direction: column-reverse; gap: 1rem;",
            theme.get_colors().thirdly,
            theme.get_colors().thirdly
        )
    }

    fn link(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; font-family: monospace; font-size: 0.75rem; @media (min-width: 768px) {{ font-size: 0.875rem; }} &:hover {{ color: {}; }}",
            theme.get_colors().primary,
            theme.get_colors().thirdly
        )
    }

    fn field(&self, theme: &impl Shading) -> String {
        format!(
            "font-family: monospace; font-size: 0.75rem; @media (min-width: 768px) {{ font-size: 0.875rem; }} color: {}; border: 1px solid {}; border-radius: 0.5rem; padding: 0.5rem; background-color: rgba(0, 0, 0, 0.2); &:focus {{ outline: none; border-color: {}; }}",
            theme.get_colors().primary,
            theme.get_colors().border,
            theme.get_colors().thirdly
        )
    }

    fn icon(&self, theme: &impl Shading) -> String {
        format!(
            "color: {}; width: 3rem; height: 3rem; @media (min-width: 768px) {{ width: 5rem; height: 5rem; }}",
            theme.get_colors().thirdly
        )
    }
}
