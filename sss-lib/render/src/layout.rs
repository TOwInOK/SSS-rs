use core::error;

use encre_css::{Config, Preflight};
use sss_core::Settings;

use crate::theme::Theme;

pub trait GetData<'a, 'b> {
    fn regular_font() -> (&'static str, &'static str) {
        (
            "PT Serif",
            "https://fonts.googleapis.com/css2?family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap",
        )
    }
    fn mono_font() -> (&'static str, &'static str) {
        (
            "PT Mono",
            "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
        )
    }
    /// Get data to render
    fn get_data(&self) -> &Settings;
    /// Get theme to theming
    fn get_theme(&self) -> &Theme;
    /// Get css config
    fn get_encre_css_config() -> Config {
        let mut config = encre_css::Config::default();
        config.preflight = Preflight::new_full()
            .font_family_mono(Self::mono_font().0.to_string())
            .font_family_sans(Self::regular_font().0.to_string());
        config
    }
}

pub trait Layout<'a, 'b>
where
    Self: GetData<'a, 'b>,
{
    /// Render layout
    fn render(&self) -> Result<String, Box<dyn error::Error + Send + Sync>>;
}

pub trait Finalize<'a, 'b>
where
    Self: Layout<'a, 'b>,
{
    /// Packing render to out format
    fn finalize(&self) -> Result<String, Box<dyn error::Error + Send + Sync>>;
}
