use encre_css::Config;
use sss_core::Settings;

use crate::theme::Shade;

pub trait GetSetData<'a, 'b, 'c, Data = Settings, Theme = crate::theme::Theme> {
    /// Get data to render
    fn get_data(&self) -> &Data;
    /// Get theme to theming
    fn get_theme(&self) -> &Theme;
    /// Get css config
    fn get_encre_css_config(&self) -> &Config;
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
    /// Set encre_config
    fn encre_css_config(
        self,
        config: &'c Config,
    ) -> Self;
}

pub trait Layout<'a, 'b, 'c, Out, Data = Settings, Theme = crate::theme::Theme>
where
    Theme: Shade,
    Self: GetSetData<'a, 'b, 'c>,
{
    /// Render layout
    fn render(&self) -> Out;
}

pub trait Finalize<'a, 'b, 'c, Out>
where
    Self: Layout<'a, 'b, 'c, Out>,
{
    /// Packing render to out format
    fn finalize(&self) -> Out;
}
