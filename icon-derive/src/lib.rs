//! Proc macro to create icon enums with embedded SVG content
//!
//! This macro downloads SVG icons during compilation and embeds them in the binary.
//! Currently supports Tabler icons downloaded from the upstream repository.
//!
//! To find the name of the icon use [tabler](https://tabler.io/icons)
//!
//! all cached in target/icon_cache for 30 days
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use tabler_icon_definer::tabler_icon;
//!
//! tabler_icon!(
//!     brand_github[outline, filled],
//!     user[outline]
//! );
//! // Generate
//! // - Tabler::OUTLINE_BRAND_GITHUB
//! // - Tabler::FILLED_BRAND_GITHUB
//! // - Tabler::OUTLINE_USER
//! ```
//!
//! With custom name and attributes:
//! ```ignore
//! use tabler_icon_definer::tabler_icon;
//!
//! tabler_icon!(
//!     #[derive(serde::Serialize)]
//!     #[name = "github"]
//!     brand_github[outline, filled],
//!     user[outline]
//! );
//! // Generate
//! // - Tabler::OUTLINE_GITHUB
//! // - Tabler::FILLED_GITHUB
//! // - Tabler::OUTLINE_USER
//! ```
//!
//! Using generated enum:
//! ```rust
//! # use tabler_icon_definer::tabler_icon;
//! # tabler_icon!(#[name = "github"] brand_github[outline, filled]);
//! let icon = Tabler::OUTLINE_GITHUB;
//! let svg = icon.as_str(); // Get SVG content
//! let name = icon.as_str_merget(); // Get icon name without style prefix
//! let full_name = icon.to_string(); // Get icon name with style prefix
//! ```
//!
//! # Limitations
//! - first item is default item

/// Downloading and cache primitives used during macro expansion.
mod cache;
/// Code generation for the final `Tabler` enum and its impl blocks.
mod codegen;
/// Parsing and normalization of the macro input AST.
mod input;
/// Shared naming helpers for enum variants, string keys, and download names.
mod naming;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::{codegen::expand_tabler_icon, input::TablerInput};

/// Proc macro to create icon enums with embedded SVG content
///
/// This macro downloads SVG icons during compilation and embeds them in the binary.
/// Currently supports Tabler icons downloaded from the upstream repository.
///
/// To find the name of the icon use [tabler](https://tabler.io/icons)
///
/// All cached in target/icon_cache for 30 days
///
/// # Examples
///
/// Basic usage:
/// ```ignore
/// use tabler_icon_definer::tabler_icon;
///
/// tabler_icon!(
///     brand_github[outline, filled],
///     user[outline]
/// );
/// // Generates:
/// // - Tabler::OUTLINE_BRAND_GITHUB
/// // - Tabler::FILLED_BRAND_GITHUB
/// // - Tabler::OUTLINE_USER
/// ```
///
/// With custom name:
/// ```ignore
/// use tabler_icon_definer::tabler_icon;
///
/// tabler_icon!(
///     #[name = "github"]
///     brand_github[outline, filled]
/// );
/// // Generates:
/// // - Tabler::OUTLINE_GITHUB
/// // - Tabler::FILLED_GITHUB
/// ```
///
/// With additional derives:
/// ```ignore
/// use tabler_icon_definer::tabler_icon;
///
/// tabler_icon!(
///     #[derive(serde::Serialize)]
///     #[name = "github"]
///     brand_github[outline, filled],
///     user[outline]
/// );
/// ```
/// Using generated enum:
/// ```ignore
/// let icon = Tabler::OUTLINE_GITHUB;
/// let svg = icon.as_str(); // Get SVG content
/// let name = icon.as_str_merget(); // Get icon name without style prefix
/// let full_name = icon.to_string(); // Get icon name with style prefix
/// ```
///
/// The macro remains exported from the crate root, while the implementation is
/// split across dedicated internal modules for parsing, naming, caching, and code generation.
#[proc_macro]
pub fn tabler_icon(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TablerInput).normalize();
    expand_tabler_icon(input).into()
}
