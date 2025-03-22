use proc_macro::TokenStream;
use quote::quote;
mod colors;
mod font;
mod node;
mod theme;

#[proc_macro]
pub fn generate_theme(input: TokenStream) -> TokenStream {
    let node = syn::parse_macro_input!(input as node::Node);
    let tree = node.generate();
    quote! {
        #tree
    }
    .into()
}
