use core::error;

use sss_core::Settings;

use crate::theme::Theme;

/// Base data for [Layout] and [Finalyze]
pub trait AdditionTeraData {
    /// Get data to render
    fn get_data(&self) -> &Settings;
    /// Get theme to theming
    fn get_theme(&self) -> &Theme;
    /// Return Card/Component layout
    fn get_card_layout(&self) -> &str;
    /// Return main layout
    fn get_layout(&self) -> &str;
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
pub trait Finalise
where
    Self: Layout,
{
    /// Packing render to out format
    fn finalize(&self) -> Result<String>;
}

/// Local result with [std::error::Error] under Box with Send + Sync
pub type Result<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;
