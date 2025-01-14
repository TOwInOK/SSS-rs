use serde::Serialize;

/// Represents hexadecimal color values as static string references.
/// These are used to define theme colors throughout the application.
/// The values should be valid hex color codes like "#FF0000".
type Color = &'static str;

/// Defines the overall theme configuration by combining colors, padding values,
/// and gap settings into a single coherent theme structure.
/// This serves as the root theme configuration object.
#[derive(Serialize, Default, Debug)]
pub struct Theme {
    pub colors: Colors,
    pub gfont_regular: (&'static str, &'static str),
    pub gfont_mono: (&'static str, &'static str),
}

/// Contains the complete color palette configuration for a theme.
/// Defines colors for different hierarchy levels (primary, secondary, tertiary)
/// as well as border colors.
/// All colors are specified as hex color codes.
#[derive(Default, Serialize, Debug)]
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

/// Defines core functionality required for theme implementation.
/// Implementors must be thread-safe (Sync + Send) and support
/// debugging and default initialization.
pub trait Shade: Sync + Send {
    /// Retrieves the color configuration for the theme
    /// Returns a reference to the Colors struct containing the theme's color palette
    fn get_colors(&self) -> &Colors;
}

/// Implements the Shading trait for the Theme struct, providing
/// access to colors, paddings, and gaps configurations.
impl Shade for Theme {
    /// Returns a reference to this theme's Colors configuration
    fn get_colors(&self) -> &Colors {
        &self.colors
    }
}
