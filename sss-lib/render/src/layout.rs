use core::error;

use encre_css::{Config, Preflight};
use sss_core::Settings;

use crate::theme::Theme;

/// Base data for [Layout] and [Finalyze]
pub trait AdditionTeraData {
    #[inline]
    fn regular_font() -> (&'static str, &'static str) {
        (
            "PT Serif",
            "https://fonts.googleapis.com/css2?family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap",
        )
    }
    #[inline]
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
    /// Return Card/Component layout
    fn get_card_layout(&self) -> &str;
    /// Return main layout
    fn get_layout(&self) -> &str;

    #[inline]
    /// Get css config
    fn get_encre_css_config() -> Config {
        let mut config = encre_css::Config::default();
        config.preflight = Preflight::new_full()
            .font_family_mono(Self::mono_font().0.to_string())
            .font_family_sans(Self::regular_font().0.to_string());
        config
    }
}

/// Allow to build a component to string
pub trait Layout
where
    Self: AdditionTeraData,
{
    /// Render layout
    fn render(&self) -> Result<String>;
}

/// Allow to finaly build a component to string via pushing [Layout] into Final Layout
pub trait Finalize
where
    Self: Layout,
{
    /// Packing render to out format
    fn finalize(&self) -> Result<String>;
}

/// Local result with [std::error::Error] under Box with Send + Sync
pub type Result<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;
