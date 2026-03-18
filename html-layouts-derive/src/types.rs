//! Internal syntax and data types used by the layout proc-macro.
//!
//! This module parses the macro input and stores the normalized metadata that later code
//! generation stages operate on.

use proc_macro2::Span;
use sss_core::LayoutLimitations;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

#[derive(Debug)]
/// Parsed arguments passed to the `generate_layouts!` proc-macro.
pub struct LayoutsInput {
    /// Path to the directory that contains card layout subdirectories.
    pub template_dir_path: LitStr,
    /// Path to the directory that contains default wrapper layout subdirectories.
    pub default_layout: LitStr,
}

impl Parse for LayoutsInput {
    /// Parses the macro input as exactly two comma-separated string literals.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the full list first so we can validate the argument count before extracting values.
        let content = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;

        // The proc-macro API is intentionally strict: callers must provide both required paths.
        if content.len() != 2 {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "expected exactly 2 string literals: template directory and default layout directory, got {}",
                    content.len()
                ),
            ));
        }

        // Consume the validated arguments in order so the rest of the crate can use named fields.
        let mut iter = content.into_iter();
        let template_dir_path = iter.next().expect("validated template directory path");
        let default_layout = iter
            .next()
            .expect("validated default layout directory path");

        Ok(LayoutsInput {
            template_dir_path,
            default_layout,
        })
    }
}

/// Parsed metadata for one concrete card layout directory.
pub struct Templates {
    /// Normalized lowercase name used for string-based APIs.
    pub layout_name: LitStr,
    /// Uppercase identifier used as the generated enum variant.
    pub layout_ident: Ident,
    /// Name of the generated constant that stores parsed limitations for this layout.
    pub const_limitations_name: Ident,
    /// Raw Tera template source loaded from `card.html.tera`.
    pub template: String,
    /// Parsed layout limitations loaded from `card.limitations.ron`.
    pub limitations: LayoutLimitations,
}

/// Parsed metadata for one default outer layout directory.
pub struct DefaultLayout {
    /// Normalized lowercase name used for string-based APIs.
    pub layout_name: LitStr,
    /// Uppercase identifier used as the generated enum variant.
    pub layout_ident: Ident,
    /// Raw Tera template source loaded from `default_layout.html.tera`.
    pub template: String,
}

#[cfg(test)]
mod tests {
    //! Tests for proc-macro input parsing.

    use syn::parse_str;

    use super::LayoutsInput;

    #[test]
    /// Verifies that the proc-macro accepts exactly two string literal paths.
    fn parses_two_paths() {
        // The parser should preserve argument order because each string literal has a distinct role.
        let input: LayoutsInput =
            parse_str("\"templates\", \"default-layouts\"").expect("input should parse");

        assert_eq!(input.template_dir_path.value(), "templates");
        assert_eq!(input.default_layout.value(), "default-layouts");
    }

    #[test]
    /// Verifies that calls with the wrong number of arguments are rejected.
    fn rejects_invalid_argument_count() {
        // Supplying a single path should fail because the macro also needs the default layout root.
        let error = parse_str::<LayoutsInput>("\"templates\"")
            .expect_err("input with one path should fail");

        assert!(
            error
                .to_string()
                .contains("expected exactly 2 string literals")
        );
    }
}
