//! Proc-macro entry point for generating HTML layout enums from on-disk Tera templates.
//!
//! The crate is consumed by `sss-std` to scan layout directories at compile time and emit
//! strongly typed enums together with the trait implementations that the renderer expects.

use std::path::Path;

use generate::generate_stream;
use generate_default_layouts::generate_default_layout_stream;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use types::LayoutsInput;

mod discovery;
mod generate;
mod generate_default_layouts;
mod types;

/// Generates layout enums and helper trait implementations from template directories.
///
/// The macro expects exactly two string literals:
/// - a directory with per-layout subdirectories that contain `card.html.tera` and
///   `card.limitations.ron`;
/// - a directory with per-layout subdirectories that contain `default_layout.html.tera`.
///
/// The generated output includes the `HtmlLayouts` and `DefaultTemplates` enums together with
/// `Layout`, `Limitations`, `Display`, `FromStr`, and helper implementations used by `sss-std`.
#[proc_macro]
pub fn generate_layouts(input: TokenStream) -> TokenStream {
    // Parse the macro arguments first so invalid input is rejected before any filesystem access.
    let data = parse_macro_input!(input as LayoutsInput);
    let template_dir_path = data.template_dir_path.value();
    let default_layout_path = data.default_layout.value();

    // Generate regular card layouts and default wrapper layouts independently so each branch can
    // report precise compile-time errors for its own source directory.
    let template_content = match generate_stream(Path::new(&template_dir_path)) {
        Ok(templates) => templates,
        Err(e) => return e.to_compile_error().into(),
    };
    let default_template_content =
        match generate_default_layout_stream(Path::new(&default_layout_path)) {
            Ok(defaults) => defaults,
            Err(e) => return e.to_compile_error().into(),
        };

    // Emit the imports required by the generated code together with both generated token streams.
    quote! {
        use render::prelude::{Layout, Limitations};
        use serde::{Deserialize, Serialize};
        use sss_core::LayoutLimitations;
        use std::borrow::Cow;
        use std::fmt;

        #template_content
        #default_template_content
    }
    .into()
}
