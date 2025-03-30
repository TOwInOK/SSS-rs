use std::borrow::Cow;

use render::prelude::*;
use sss_core::{types::provider::Tabler, *};
use theme::Theme;

use super::{builder::HtmlTeraRender, final_builder::HtmlTeraFinalize};

/// Tera data simplification
pub trait TeraData {
    /// Get data to render
    fn get_data(&self) -> &Data;
    /// Get theme to theming
    fn get_theme(&self) -> &Theme;
    /// Return layout
    fn get_layout(&self) -> Cow<String>;
    /// Return component
    fn get_limitations(&self) -> Option<Cow<LayoutLimitations>>;
}

impl<L: Layout<String> + Clone> TeraData for HtmlTeraRender<'_, '_, '_, L> {
    #[inline]
    /// Return data from component
    fn get_data(&self) -> &Data {
        self.settings.as_ref()
    }

    #[inline]
    /// Return theme from component
    fn get_theme(&self) -> &Theme {
        self.theme.as_ref()
    }

    #[inline]
    /// Return template of component
    fn get_layout(&self) -> Cow<String> {
        self.layout.template()
    }

    #[inline]
    /// Return limitations of component layout
    fn get_limitations(&self) -> Option<Cow<LayoutLimitations>> {
        self.layout.limitations()
    }
}

impl<C: Component<String> + TeraData + Clone, L: Layout<String> + Clone> TeraData
    for HtmlTeraFinalize<'_, '_, C, L>
{
    #[inline]
    /// Return data from component
    fn get_data(&self) -> &Data {
        self.component.get_data()
    }

    #[inline]
    /// Return theme from component
    fn get_theme(&self) -> &Theme {
        self.component.get_theme()
    }

    #[inline]
    /// Return template of final layout
    fn get_layout(&self) -> Cow<String> {
        self.layout.template()
    }

    #[inline]
    /// Return limitations of final layout
    fn get_limitations(&self) -> Option<Cow<LayoutLimitations>> {
        self.layout.limitations()
    }
}

/// Return svg [String] by icon name
/// ```tera
///  {{ get_svg(icon=social.icon) | safe }}
/// ```
pub fn get_svg(args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    // Получаем значение provider из аргументов
    let provider = match args.get("icon") {
        Some(val) => val.to_string(),
        None => {
            return Err(tera::Error::msg(
                "Icon argument is required. Check your function invocation in template",
            ));
        }
    };

    // Убираем кавычки и пробелы
    let provider = provider.trim_matches('"').trim();

    // Преобразуем строку в Provider enum
    let provider: Tabler = provider.parse()?;

    // Вызываем as_ref() и возвращаем SVG как строку
    Ok(tera::Value::String(provider.as_str().to_string()))
}

/// Return true name of icon
/// ```tera
///  {{ get_icon_name(icon=social.icon) }}
/// ```
pub fn get_icon_name(
    args: &std::collections::HashMap<String, tera::Value>
) -> tera::Result<tera::Value> {
    // Получаем значение provider из аргументов
    let provider = match args.get("icon") {
        Some(val) => val.to_string(),
        None => {
            return Err(tera::Error::msg(
                "Icon argument is required. Check your function invocation in template",
            ));
        }
    };

    // Убираем кавычки и пробелы
    let provider = provider.trim_matches('"').trim();

    // Преобразуем строку в Provider enum
    let provider: Tabler = provider.parse()?;

    // Вызываем as_ref() и возвращаем SVG как строку
    Ok(tera::Value::String(provider.as_str_merget().to_string()))
}
