//! Parsing and code generation utilities for a single theme declaration.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Token, parse::Parse};

use crate::{colors::ColorsLayout, font::FontLayout};

/// Parsed representation of one theme entry from the macro input.
pub struct ThemeLayout {
    pub name: Ident,
    pub colors: ColorsLayout,
    pub font: FontLayout,
}

impl Parse for ThemeLayout {
    /// Parses a single theme block containing `colors` and `font` sections.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Read the theme name first so it can later become the generated static identifier.
        let name = input.parse()?;

        // Enter the theme body and parse nested sections from its own token stream.
        let content;
        syn::braced!(content in input);

        // Parse the required colors section before any other entries.
        let colors = content.parse()?;

        // The grammar requires a comma between the colors and font sections.
        if !content.peek(Token![,]) {
            return Err(syn::Error::new(
                content.span(),
                "Expected comma after colors section",
            ));
        }
        content.parse::<Token![,]>()?;

        // Parse the required font section after the separating comma.
        let font = content.parse()?;

        // Reject any trailing content so the macro input stays strict and predictable.
        if !content.is_empty() {
            return Err(syn::Error::new(
                content.span(),
                "Unexpected content after font section",
            ));
        }

        Ok(ThemeLayout {
            name,
            colors,
            font,
        })
    }
}

impl ThemeLayout {
    /// Generates a static `Theme` definition for this parsed theme entry.
    pub fn generate(&self) -> TokenStream {
        // Normalize the theme name into the uppercase identifier style used by generated statics.
        let name = syn::Ident::new(&self.name.to_string().to_uppercase(), self.name.span());
        // Generate the nested colors and font fragments once before assembling the full item.
        let colors = &self.colors.generate();
        let font = &self.font.generate();

        // Emit a `static` item so other generated helpers can reference the theme by name.
        quote! {
            /// Static theme definition generated from the macro input.
            pub static #name: Theme = Theme {
                #colors,
                #font,
            };
        }
    }
}
