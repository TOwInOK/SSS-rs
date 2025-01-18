use encre_css::{Config, Preflight};
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
    pub regular_font: (&'static str, &'static str),
    pub mono_font: (&'static str, &'static str),
}

/// Contains the complete color palette configuration for a theme.
/// Defines colors for different hierarchy levels (primary, secondary, tertiary)
/// as well as border colors.
/// All colors are specified as hex color codes.
#[derive(Default, Serialize, Debug)]
pub struct Colors {
    /// Primary theme color used for main (like text) UI elements
    pub primary: Color,
    /// Secondary theme color for background elements
    pub secondary: Color,
    /// Tertiary theme color for additional accent elements
    pub thirdly: Color,
    /// Color used for borders, texts and separators
    pub border: Color,
}

/// Defines core functionality required for theme implementation.
/// Implementors must be thread-safe (Sync + Send) and support
/// debugging and default initialization.
pub trait Shade: Sync + Send {
    /// Retrieves the color configuration for the theme
    /// Returns a reference to the Colors struct containing the theme's color palette
    fn get_colors(&self) -> &Colors;
    fn regular_font(&self) -> (&'static str, &'static str);
    fn mono_font(&self) -> (&'static str, &'static str);

    #[inline]
    /// Get css config
    fn get_encre_css_config(&self) -> Config {
        let mut config = encre_css::Config::default();
        config.preflight = Preflight::new_full()
            .font_family_mono(self.mono_font().0.to_string())
            .font_family_sans(self.regular_font().0.to_string());
        config
    }
}

/// Implements the Shading trait for the Theme struct, providing
/// access to colors, paddings, and gaps configurations.
impl Shade for Theme {
    /// Returns a reference to this theme's Colors configuration
    #[inline]
    fn get_colors(&self) -> &Colors {
        &self.colors
    }
    #[inline]
    fn regular_font(&self) -> (&'static str, &'static str) {
        self.regular_font
    }
    #[inline]
    fn mono_font(&self) -> (&'static str, &'static str) {
        self.mono_font
    }
}
