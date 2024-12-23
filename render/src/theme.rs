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
pub enum Shade<T: TailwindShading + CssShading> {
    Dark(T),
    Light(T),
}

impl<T: Default + TailwindShading + CssShading> Default for Shade<T> {
    fn default() -> Self {
        Self::Dark(T::default())
    }
}

/// generate some classes for specific elements
/// overwise need to use css prefix (main.css)
pub trait TailwindShading
where
    Self: Sync + Send + Debug + Default,
{
    fn body(&self) -> String;
    fn label(&self) -> String;
    fn sub_label(&self) -> String;
    fn text(&self) -> String;
    fn text_minor(&self) -> String;
    fn horizontal_frame(&self) -> String;
    fn reversed_horizontal_frame(&self) -> String;
    fn vertical_frame(&self) -> String;
    fn reversed_vertical_frame(&self) -> String;
    fn link(&self) -> String;
    fn field(&self) -> String;
    fn icon(&self) -> String;
}

impl TailwindShading for Theme {
    fn body(&self) -> String {
        format!("bg-[{}]", self.color.thirdly)
    }

    fn label(&self) -> String {
        format!("text-[{}] font-semibold text-lg", self.color.primary)
    }

    fn sub_label(&self) -> String {
        format!("text-[{}] font-medium text-base", self.color.secondary)
    }

    fn text(&self) -> String {
        format!("text-[{}] text-base", self.color.primary)
    }

    fn text_minor(&self) -> String {
        format!("text-[{}] text-sm", self.color.thirdly)
    }

    fn horizontal_frame(&self) -> String {
        format!(
            "flex flex-row border-[{}] border-[{}px] rounded-md p-{} gap-{}",
            self.color.border, self.padding.border, self.padding.frame, self.gap.frame
        )
    }

    fn reversed_horizontal_frame(&self) -> String {
        format!(
            "flex flex-row-reverse border-[{}] border-[{}px] rounded-md p-{} gap-{}",
            self.color.border, self.padding.border, self.padding.frame, self.gap.frame
        )
    }

    fn vertical_frame(&self) -> String {
        format!(
            "flex flex-col border-[{}] border-[{}px] rounded-md p-{} gap-{}",
            self.color.border, self.padding.border, self.padding.frame, self.gap.frame
        )
    }

    fn reversed_vertical_frame(&self) -> String {
        format!(
            "flex flex-col-reverse border-[{}] border-[{}px] rounded-md p-{} gap-{}",
            self.color.border, self.padding.border, self.padding.frame, self.gap.frame
        )
    }

    fn link(&self) -> String {
        format!(
            "text-[{}] hover:underline cursor-pointer",
            self.color.primary
        )
    }

    fn field(&self) -> String {
        format!(
            "border-[{}] border-[{}px] rounded p-{} focus:outline-none focus:border-[{}]",
            self.color.border, self.padding.border, self.padding.button, self.color.primary
        )
    }

    fn icon(&self) -> String {
        format!("w-6 h-6 text-[{}]", self.color.primary)
    }
}

impl<T: TailwindShading + CssShading> TailwindShading for Shade<T> {
    fn body(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::body(theme),
        }
    }

    fn label(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::label(theme),
        }
    }

    fn sub_label(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::sub_label(theme),
        }
    }

    fn text(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::text(theme),
        }
    }

    fn text_minor(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::text_minor(theme),
        }
    }

    fn horizontal_frame(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::horizontal_frame(theme),
        }
    }

    fn reversed_horizontal_frame(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => {
                TailwindShading::reversed_horizontal_frame(theme)
            }
        }
    }

    fn vertical_frame(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::vertical_frame(theme),
        }
    }

    fn reversed_vertical_frame(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => {
                TailwindShading::reversed_vertical_frame(theme)
            }
        }
    }

    fn link(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::link(theme),
        }
    }

    fn field(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::field(theme),
        }
    }

    fn icon(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => TailwindShading::icon(theme),
        }
    }
}

pub trait CssShading
where
    Self: Sync + Send + Debug + Default,
{
    fn body(&self) -> String;
    fn label(&self) -> String;
    fn sub_label(&self) -> String;
    fn text(&self) -> String;
    fn text_minor(&self) -> String;
    fn horizontal_frame(&self) -> String;
    fn reversed_horizontal_frame(&self) -> String;
    fn vertical_frame(&self) -> String;
    fn reversed_vertical_frame(&self) -> String;
    fn link(&self) -> String;
    fn field(&self) -> String;
    fn icon(&self) -> String;
}

impl<T: CssShading + TailwindShading> CssShading for Shade<T> {
    fn body(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::body(theme),
        }
    }

    fn label(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::label(theme),
        }
    }

    fn sub_label(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::sub_label(theme),
        }
    }

    fn text(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::text(theme),
        }
    }

    fn text_minor(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::text_minor(theme),
        }
    }

    fn horizontal_frame(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::horizontal_frame(theme),
        }
    }

    fn reversed_horizontal_frame(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => {
                CssShading::reversed_horizontal_frame(theme)
            }
        }
    }

    fn vertical_frame(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::vertical_frame(theme),
        }
    }

    fn reversed_vertical_frame(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::reversed_vertical_frame(theme),
        }
    }

    fn link(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::link(theme),
        }
    }

    fn field(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::field(theme),
        }
    }

    fn icon(&self) -> String {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => CssShading::icon(theme),
        }
    }
}

impl CssShading for Theme {
    fn body(&self) -> String {
        format!("background-color: {};", self.color.thirdly)
    }

    fn label(&self) -> String {
        format!(
            "color: {}; font-weight: 600; font-size: 1.125rem; line-height: 1.75rem;",
            self.color.primary
        )
    }

    fn sub_label(&self) -> String {
        format!(
            "color: {}; font-weight: 500; font-size: 1rem; line-height: 1.5rem;",
            self.color.secondary
        )
    }

    fn text(&self) -> String {
        format!(
            "color: {}; font-size: 1rem; line-height: 1.5rem;",
            self.color.primary
        )
    }

    fn text_minor(&self) -> String {
        format!(
            "color: {}; font-size: 0.875rem; line-height: 1.25rem;",
            self.color.thirdly
        )
    }

    fn horizontal_frame(&self) -> String {
        format!(
            "display: flex; flex-direction: row; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            self.padding.border,
            self.color.border,
            self.padding.frame,
            self.gap.frame
        )
    }

    fn reversed_horizontal_frame(&self) -> String {
        format!(
            "display: flex; flex-direction: row-reverse; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            self.padding.border,
            self.color.border,
            self.padding.frame,
            self.gap.frame
        )
    }

    fn vertical_frame(&self) -> String {
        format!(
            "display: flex; flex-direction: column; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            self.padding.border,
            self.color.border,
            self.padding.frame,
            self.gap.frame
        )
    }

    fn reversed_vertical_frame(&self) -> String {
        format!(
            "display: flex; flex-direction: column-reverse; border: {}px solid {}; border-radius: 0.375rem; padding: {}px; gap: {}px;",
            self.padding.border,
            self.color.border,
            self.padding.frame,
            self.gap.frame
        )
    }

    fn link(&self) -> String {
        format!(
            "color: {}; text-decoration: none; cursor: pointer;",
            self.color.primary
        )
    }

    fn field(&self) -> String {
        format!(
            "border: {}px solid {}; border-radius: 0.25rem; padding: {}px; outline: none;",
            self.padding.border, self.color.border, self.padding.button
        )
    }

    fn icon(&self) -> String {
        format!(
            "width: 1.5rem; height: 1.5rem; color: {};",
            self.color.primary
        )
    }
}
