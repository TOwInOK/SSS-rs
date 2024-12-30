/// Represents hexadecimal color values as static string references.
/// These are used to define theme colors throughout the application.
/// The values should be valid hex color codes like "#FF0000".
type Color = &'static str;

/// Defines size measurements in pixels for spacing between UI elements.
/// Used to control the internal padding of components.
/// Values are specified as positive integers representing pixel counts.
type Padding = usize;

/// Specifies spacing measurements in pixels between distinct UI components.
/// Controls the external spacing between different elements.
/// Values are specified as positive integers representing pixel counts.
type Gap = usize;

/// Defines the overall theme configuration by combining colors, padding values,
/// and gap settings into a single coherent theme structure.
/// This serves as the root theme configuration object.
#[derive(Default)]
pub struct Theme {
    pub color: Colors,
    pub padding: Paddings,
    pub gap: Gaps,
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

/// Specifies padding measurements for different UI element types.
/// Controls internal spacing for buttons, borders, and frames.
/// All measurements are in pixels.
#[derive(Default)]
pub struct Paddings {
    /// Internal padding for button elements
    pub button: Padding,
    /// Padding around border elements
    pub border: Padding,
    /// Internal padding for frame containers
    pub frame: Padding,
}

/// Defines spacing measurements between distinct UI components.
/// Currently supports frame-level gap configuration.
/// All measurements are in pixels.
#[derive(Default)]
pub struct Gaps {
    /// Spacing between frame elements
    pub frame: Gap,
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

    /// Retrieves the padding configuration for the theme
    /// Returns a reference to the Paddings struct containing padding measurements
    fn get_paddings(&self) -> &Paddings;

    /// Retrieves the gap configuration for the theme
    /// Returns a reference to the Gaps struct containing spacing measurements
    fn get_gaps(&self) -> &Gaps;

    // for body
    /// some meta tags
    fn get_meta(&self) -> &str;
    /// some tailwind or any scripts
    fn get_scripts(&self) -> &str;
    /// fonts
    fn get_links(&self) -> &str;
}

/// Implements the Shading trait for the Theme struct, providing
/// access to colors, paddings, and gaps configurations.
impl Shading for Theme {
    /// Returns a reference to this theme's Colors configuration
    fn get_colors(&self) -> &Colors {
        &self.color
    }

    /// Returns a reference to this theme's Paddings configuration
    fn get_paddings(&self) -> &Paddings {
        &self.padding
    }

    /// Returns a reference to this theme's Gaps configuration
    fn get_gaps(&self) -> &Gaps {
        &self.gap
    }

    fn get_meta(&self) -> &str {
        todo!()
    }

    fn get_scripts(&self) -> &str {
        todo!()
    }

    fn get_links(&self) -> &str {
        todo!()
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

    /// Returns padding configuration from the contained theme
    fn get_paddings(&self) -> &Paddings {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => theme.get_paddings(),
        }
    }

    /// Returns gap configuration from the contained theme
    fn get_gaps(&self) -> &Gaps {
        match self {
            Shade::Dark(theme) | Shade::Light(theme) => theme.get_gaps(),
        }
    }

    fn get_meta(&self) -> &str {
        todo!()
    }

    fn get_scripts(&self) -> &str {
        todo!()
    }

    fn get_links(&self) -> &str {
        todo!()
    }
}
