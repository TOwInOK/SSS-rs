//! Proc-macro crate for generating static theme definitions and helper APIs.

use proc_macro::TokenStream;
use quote::quote;
mod colors;
mod font;
mod node;
mod theme;

#[proc_macro]
/// Parses the theme DSL and expands it into generated Rust items.
pub fn generate_theme(input: TokenStream) -> TokenStream {
    // Parse the macro input into the internal syntax tree that describes all themes.
    let node = syn::parse_macro_input!(input as node::Node);
    // Ask the parsed tree to build the final set of generated items.
    let tree = node.generate();
    // Return the generated tokens back to the compiler as the macro expansion result.
    quote! {
        #tree
    }
    .into()
}
