use std::fmt::Debug;

pub mod umbrella;

/// Represents hexadecimal color values as static string references.
/// These are used to define theme colors throughout the application.
type Color = &'static str;
/// Defines size measurements in pixels for spacing between UI elements.
/// Used to control the internal padding of components.
type Padding = usize;
/// Specifies spacing measurements in pixels between distinct UI components.
/// Controls the external spacing between different elements.
type Gap = usize;

/// Defines the overall theme configuration by combining colors, padding values,
/// and gap settings into a single coherent theme structure.
#[derive(Debug, Default)]
pub struct Theme {
    pub color: Colors,
    pub padding: Paddings,
    pub gap: Gaps,
}

/// Contains the complete color palette configuration for a theme.
/// Defines colors for different hierarchy levels (primary, secondary, tertiary)
/// as well as border colors.
#[derive(Debug, Default)]
pub struct Colors {
    pub primary: Color,
    pub secondary: Color,
    pub thirdly: Color,
    pub border: Color,
}

/// Specifies padding measurements for different UI element types.
/// Controls internal spacing for buttons, borders, and frames.
#[derive(Debug, Default)]
pub struct Paddings {
    pub button: Padding,
    pub border: Padding,
    pub frame: Padding,
}

/// Defines spacing measurements between distinct UI components.
/// Currently supports frame-level gap configuration.
#[derive(Debug, Default)]
pub struct Gaps {
    pub frame: Gap,
}

/// Represents theme variants that support both dark and light modes.
/// Generic parameter T must implement the Shading trait to provide
/// necessary theming functionality.
#[derive(Debug)]
pub enum Shade<T: Shading> {
    Dark(T),
    Light(T),
}

/// Provides default implementation for theme shading, setting dark mode
/// as the default variant. Requires type T to implement both Default and
/// Shading traits.
impl<T: Default + Shading> Default for Shade<T> {
    fn default() -> Self {
        Self::Dark(T::default())
    }
}

/// Defines core functionality required for theme implementation.
/// Implementors must be thread-safe (Sync + Send) and support
/// debugging and default initialization.
pub trait Shading: Sync + Send + Debug + Default {
    /// Retrieves the color configuration for the theme
    fn get_colors(&self) -> &Colors;
    /// Retrieves the padding configuration for the theme
    fn get_paddings(&self) -> &Paddings;
    /// Retrieves the gap configuration for the theme
    fn get_gaps(&self) -> &Gaps;
}

/// Implements the Shading trait for the Theme struct, providing
/// access to colors, paddings, and gaps configurations.
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
