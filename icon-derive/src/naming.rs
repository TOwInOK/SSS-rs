//! Naming helpers shared between parsing, downloading, and code generation.

use proc_macro2::Ident;
use quote::format_ident;

use crate::input::IconDefinition;

/// Returns the user-facing icon name used in generated variants and merged labels.
pub(crate) fn display_name(icon: &IconDefinition) -> String {
    icon.display_name
        .clone()
        .unwrap_or_else(|| icon.name.trim_end_matches('_').to_string())
}

/// Converts the Rust identifier-like icon name into the download slug expected by Tabler.
pub(crate) fn download_name(icon: &IconDefinition) -> String {
    icon.name
        .trim_end_matches('_')
        .to_lowercase()
        .replace('_', "-")
}

/// Builds an enum variant identifier such as `OUTLINE_BRAND_GITHUB`.
pub(crate) fn variant_ident(
    style: &str,
    display_name: &str,
) -> Ident {
    format_ident!("{}_{}", style.to_uppercase(), display_name.to_uppercase())
}

/// Builds the lowercase string key used by [`Display`](std::fmt::Display) and [`FromStr`](std::str::FromStr).
pub(crate) fn icon_key(
    style: &str,
    display_name: &str,
) -> String {
    format!("{}_{}", style.to_lowercase(), display_name.to_lowercase())
}
