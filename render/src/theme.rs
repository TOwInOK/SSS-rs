/// Represents hexadecimal color values as static string references.
/// These are used to define theme colors throughout the application.
/// The values should be valid hex color codes like "#FF0000".
type Color = &'static str;

/// Defines the overall theme configuration by combining colors, padding values,
/// and gap settings into a single coherent theme structure.
/// This serves as the root theme configuration object.
#[derive(Default)]
pub struct Theme {
    pub color: Colors,
}

/// Contains the complete color palette configuration for a theme.
/// Defines colors for different hierarchy levels (primary, secondary, tertiary)
/// as well as border colors.
/// All colors are specified as hex color codes.
#[derive(Default)]
pub struct Colors {
    /// Primary theme color used for main UI elements
    pub primary: Color,
    /// Secondary theme color for supporting UI elements
    pub secondary: Color,
    /// Tertiary theme color for additional accent elements
    pub thirdly: Color,
    /// Color used for borders and separators
    pub border: Color,
}

/// Represents theme variants that support both dark and light modes.
/// Generic parameter T must implement the Shading trait to provide
/// necessary theming functionality.
/// Used to switch between dark and light theme variants.
pub enum Shade<T: Shading> {
    /// Dark theme variant
    Dark(T),
    /// Light theme variant
    Light(T),
}

/// Provides default implementation for theme shading, setting dark mode
/// as the default variant. Requires type T to implement both Default and
/// Shading traits.
impl<T: Default + Shading> Default for Shade<T> {
    /// Returns the default dark theme variant
    fn default() -> Self {
        Self::Dark(T::default())
    }
}

/// Defines core functionality required for theme implementation.
/// Implementors must be thread-safe (Sync + Send) and support
/// debugging and default initialization.
pub trait Shading: Sync + Send + Default {
    /// Retrieves the color configuration for the theme
    /// Returns a reference to the Colors struct containing the theme's color palette
    fn get_colors(&self) -> &Colors;
}

/// Implements the Shading trait for the Theme struct, providing
/// access to colors, paddings, and gaps configurations.
impl Shading for Theme {
    /// Returns a reference to this theme's Colors configuration
    fn get_colors(&self) -> &Colors {
        &self.color
    }
}

/// Implementation of Shading trait for the Shade enum to delegate
/// theme configuration access to the contained theme instance
impl<T: Shading> Shading for Shade<T> {
    /// Returns color configuration from the contained theme
    fn get_colors(&self) -> &Colors {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => theme.get_colors(),
        }
    }
}
