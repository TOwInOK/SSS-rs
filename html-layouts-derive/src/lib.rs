use std::path::PathBuf;

use generate::generate_stream;
use generate_default_layouts::generate_default_layout_stream;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use types::LayoutsInput;

mod generate;
mod generate_default_layouts;
mod types;

#[proc_macro]
pub fn generate_layouts(input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as LayoutsInput);
    let template_content = match generate_stream(&PathBuf::from(data.template_dir_path.value())) {
        Ok(templates) => templates,
        Err(e) => return e.to_compile_error().into(),
    };
    let default_template_content =
        match generate_default_layout_stream(&PathBuf::from(data.default_layout.value())) {
            Ok(defaults) => defaults,
            Err(e) => return e.to_compile_error().into(),
        };
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
