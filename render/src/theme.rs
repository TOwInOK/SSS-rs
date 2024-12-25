use std::fmt::Debug;

pub mod umbrella;

/// Only hex
type Color = &'static str;
/// b/f
type Padding = usize;
/// after
type Gap = usize;
#[derive(Debug, Default)]
pub struct Theme {
    pub color: Colors,
    pub padding: Paddings,
    pub gap: Gaps,
}
/// Base colors
#[derive(Debug, Default)]
pub struct Colors {
    pub primary: Color,
    pub secondary: Color,
    pub thirdly: Color,
    pub border: Color,
}
/// Base padding
#[derive(Debug, Default)]
pub struct Paddings {
    pub button: Padding,
    pub border: Padding,
    pub frame: Padding,
}
/// Base gap
#[derive(Debug, Default)]
pub struct Gaps {
    pub frame: Gap,
}

#[derive(Debug)]
pub enum Shade<T: Shading> {
    Dark(T),
    Light(T),
}

impl<T: Default + Shading> Default for Shade<T> {
    fn default() -> Self {
        Self::Dark(T::default())
    }
}

pub trait Shading: Sync + Send + Debug + Default {
    fn get_colors(&self) -> &Colors;
    fn get_paddings(&self) -> &Paddings;
    fn get_gaps(&self) -> &Gaps;
}

impl Shading for Theme {
    fn get_colors(&self) -> &Colors {
        &self.color
    }

    fn get_paddings(&self) -> &Paddings {
        &self.padding
    }

    fn get_gaps(&self) -> &Gaps {
        &self.gap
    }
}

pub trait TailwindShading: Shading {
    fn body(&self) -> String {
        format!("bg-{}", self.get_colors().thirdly)
    }

    fn label(&self) -> String {
        format!("text-{} font-semibold text-lg", self.get_colors().primary)
    }

    fn sub_label(&self) -> String {
        format!("text-{} font-medium text-base", self.get_colors().secondary)
    }

    fn text(&self) -> String {
        format!("text-{} text-base", self.get_colors().primary)
    }

    fn text_minor(&self) -> String {
        format!("text-{} text-sm", self.get_colors().thirdly)
    }

    fn horizontal_frame(&self) -> String {
        format!(
            "flex flex-row border-{} border-{} rounded-md p-{} gap-{}",
            self.get_colors().border,
            self.get_paddings().border,
            self.get_paddings().frame,
            self.get_gaps().frame
        )
    }

    fn reversed_horizontal_frame(&self) -> String {
        format!(
            "flex flex-row-reverse border-{} border-{} rounded-md p-{} gap-{}",
            self.get_colors().border,
            self.get_paddings().border,
            self.get_paddings().frame,
            self.get_gaps().frame
        )
    }

    fn vertical_frame(&self) -> String {
        format!(
            "flex flex-col border-{} border-{} rounded-md p-{} gap-{}",
            self.get_colors().border,
            self.get_paddings().border,
            self.get_paddings().frame,
            self.get_gaps().frame
        )
    }

    fn reversed_vertical_frame(&self) -> String {
        format!(
            "flex flex-col-reverse border-{} border-{} rounded-md p-{} gap-{}",
            self.get_colors().border,
            self.get_paddings().border,
            self.get_paddings().frame,
            self.get_gaps().frame
        )
    }

    fn link(&self) -> String {
        format!(
            "text-{} hover:underline cursor-pointer",
            self.get_colors().primary
        )
    }

    fn field(&self) -> String {
        format!(
            "border-{} border-{} rounded p-{} focus:outline-none focus:border-{}",
            self.get_colors().border,
            self.get_paddings().border,
            self.get_paddings().button,
            self.get_colors().primary
        )
    }

    fn icon(&self) -> String {
        format!("w-6 h-6 text-{}", self.get_colors().primary)
    }
}

// CSS formatting
pub trait CssShading: Shading {
    fn body(&self) -> String {
        format!("background-color: {};", self.get_colors().thirdly)
    }

    fn label(&self) -> String {
        format!(
            "color: {}; font-weight: 600; font-size: 1.125rem; line-height: 1.75rem;",
            self.get_colors().primary
        )
    }

    fn sub_label(&self) -> String {
        format!(
            "color: {}; font-weight: 500; font-size: 1rem; line-height: 1.5rem;",
            self.get_colors().secondary
        )
    }

    fn text(&self) -> String {
        format!(
            "color: {}; font-size: 1rem; line-height: 1.5rem;",
            self.get_colors().primary
        )
    }

    fn text_minor(&self) -> String {
        format!(
            "color: {}; font-size: 0.875rem; line-height: 1.25rem;",
            self.get_colors().thirdly
        )
    }

    fn horizontal_frame(&self) -> String {
        format!(
            "display: flex; flex-direction: row; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            self.get_paddings().border,
            self.get_colors().border,
            self.get_paddings().frame,
            self.get_gaps().frame
        )
    }

    fn reversed_horizontal_frame(&self) -> String {
        format!(
            "display: flex; flex-direction: row-reverse; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            self.get_paddings().border,
            self.get_colors().border,
            self.get_paddings().frame,
            self.get_gaps().frame
        )
    }

    fn vertical_frame(&self) -> String {
        format!(
            "display: flex; flex-direction: column; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            self.get_paddings().border,
            self.get_colors().border,
            self.get_paddings().frame,
            self.get_gaps().frame
        )
    }

    fn reversed_vertical_frame(&self) -> String {
        format!(
            "display: flex; flex-direction: column-reverse; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            self.get_paddings().border,
            self.get_colors().border,
            self.get_paddings().frame,
            self.get_gaps().frame
        )
    }

    fn link(&self) -> String {
        format!(
            "color: {}; text-decoration: none; cursor: pointer;",
            self.get_colors().primary
        )
    }

    fn field(&self) -> String {
        format!(
            "border: {}px solid {}; border-radius: 0.25rem; padding: {}px; outline: none;",
            self.get_paddings().border,
            self.get_colors().border,
            self.get_paddings().button
        )
    }

    fn icon(&self) -> String {
        format!(
            "width: 1.5rem; height: 1.5rem; color: {};",
            self.get_colors().primary
        )
    }
}

// Simple implementations for Theme
impl TailwindShading for Theme {}
impl CssShading for Theme {}

// Implementations for Shade
impl<T: Shading> Shading for Shade<T> {
    fn get_colors(&self) -> &Colors {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => theme.get_colors(),
        }
    }

    fn get_paddings(&self) -> &Paddings {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => theme.get_paddings(),
        }
    }

    fn get_gaps(&self) -> &Gaps {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => theme.get_gaps(),
        }
    }
}

impl<T: Shading + TailwindShading> TailwindShading for Shade<T> {}
impl<T: Shading + CssShading> CssShading for Shade<T> {}
