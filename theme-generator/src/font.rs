//! Parsing and code generation utilities for the `font { ... }` section.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

/// Parsed representation of a theme font configuration.
pub struct FontLayout {
    pub name: FontField,
    pub link: FontField,
}

/// Parsed representation of a single field inside the font block.
pub struct FontField {
    pub name: Ident,
    _colon_token: Token![:],
    pub value: LitStr,
    _comma_token: Option<Token![,]>,
}

impl Parse for FontField {
    /// Parses a single font field written as `name: "value"`.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Read the field name, separator and literal value in declaration order.
        Ok(FontField {
            name: input.parse()?,
            _colon_token: input.parse()?,
            value: input.parse()?,
            // Accept an optional trailing comma so the block syntax matches Rust conventions.
            _comma_token: if input.peek(Token![,]) { Some(input.parse()?) } else { None },
        })
    }
}

impl Parse for FontLayout {
    /// Parses the full `font { ... }` block and validates the supported fields.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Validate that the parser is currently positioned at the `font` section.
        let font_token: Ident = input.parse()?;
        if font_token != "font" {
            return Err(input.error("expected 'font'"));
        }

        // Parse the field list inside braces so errors are localized to the font block.
        let content;
        syn::braced!(content in input);

        // Read the font name entry first and ensure the key is correct.
        let name: FontField = content.parse()?;
        if name.name != "name" {
            return Err(content.error("expected name"));
        }

        // Read the font link entry next and validate its key.
        let link: FontField = content.parse()?;
        if link.name != "link" {
            return Err(content.error("expected link"));
        }

        Ok(FontLayout { name, link })
    }
}

impl FontLayout {
    /// Builds the token stream for initializing the `font` field of a generated theme.
    pub fn generate(&self) -> TokenStream {
        // Convert parsed string literals into plain string values for quote interpolation.
        let name = &self.name.value.value();
        let link = &self.link.value.value();

        // Emit the tuple fragment expected by the generated `Theme` initializer.
        quote! {
            font: (
                #name,
                #link,
            )
        }
    }
}
