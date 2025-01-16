use core::error;

use encre_css::{Config, Preflight};
use sss_core::Settings;

use crate::theme::Shade;

pub trait GetSetData<'a, 'b, Data = Settings, Theme = crate::theme::Theme> {
    fn regular_font() -> (&'static str, &'static str);
    fn mono_font() -> (&'static str, &'static str);
    /// Get data to render
    fn get_data(&self) -> &Data;
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
    /// Set data
    fn data(
        self,
        data: &'a Data,
    ) -> Self;
    /// Set theme
    fn theme(
        self,
        theme: &'b Theme,
    ) -> Self;
}

pub trait Layout<
    'a,
    'b,
    Out = Result<String, Box<dyn error::Error + Send + Sync>>,
    Data = Settings,
    Theme = crate::theme::Theme,
> where
    Theme: Shade,
    Self: GetSetData<'a, 'b>,
{
    /// Render layout
    fn render(&self) -> Out;
}

pub trait Finalize<'a, 'b, Out = Result<String, Box<dyn error::Error + Send + Sync>>>
where
    Self: Layout<'a, 'b, Out>,
{
    /// Packing render to out format
    fn finalize(&self) -> Out;
}
