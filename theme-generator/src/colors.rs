//! Parsing and code generation utilities for the `colors { ... }` section.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

/// Parsed representation of a theme color palette.
pub struct ColorsLayout {
    pub text: ColorField,
    pub background: ColorField,
    pub accent: ColorField,
    pub border: ColorField,
}

/// Parsed representation of a single field inside the colors block.
pub struct ColorField {
    pub name: Ident,
    _separator: Token![:],
    pub value: LitStr,
    _after: Option<Token![,]>,
}

impl Parse for ColorField {
    /// Parses a single color field written as `name: "value"`.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Read the field name first so the caller can validate which entry was provided.
        let name = input.parse()?;
        // Consume the separating colon before the literal color value.
        let separator = input.parse()?;
        // Capture the color literal exactly as it was written in the macro input.
        let value = input.parse()?;
        // Allow an optional trailing comma for a more ergonomic block syntax.
        let after = input.parse()?;

        Ok(ColorField {
            name,
            _separator: separator,
            value,
            _after: after,
        })
    }
}

impl Parse for ColorsLayout {
    /// Parses the full `colors { ... }` block and validates the expected field order.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Read and validate the section name before entering the block contents.
        let colors_token: Ident = input.parse()?;

        if colors_token != "colors" {
            return Err(input.error("expected 'colors'"));
        }

        // Reject an empty section early to provide a clearer parser error.
        if input.is_empty() {
            return Err(input.error("expected text"));
        }

        // Parse the block body separately so field-level errors point to the right span.
        let content;
        syn::braced!(content in input);

        // Parse each supported field in the required order and validate its identifier.
        let text: ColorField = content.parse()?;
        if text.name != "text" {
            return Err(content.error("expected 'text' field"));
        }

        let background: ColorField = content.parse()?;
        if background.name != "background" {
            return Err(content.error("expected 'background' field"));
        }

        let accent: ColorField = content.parse()?;
        if accent.name != "accent" {
            return Err(content.error("expected 'accent' field"));
        }

        let border: ColorField = content.parse()?;
        if border.name != "border" {
            return Err(content.error("expected 'border' field"));
        }

        // Ensure the block does not contain unsupported trailing tokens.
        if !content.is_empty() {
            return Err(content.error("unexpected token"));
        }

        Ok(ColorsLayout {
            text,
            background,
            accent,
            border,
        })
    }
}

impl ColorsLayout {
    /// Builds the token stream for initializing the `colors` field of a generated theme.
    pub fn generate(&self) -> TokenStream {
        // Extract the parsed literals once so they can be interpolated into the output.
        let text = &self.text.value;
        let background = &self.background.value;
        let accent = &self.accent.value;
        let border = &self.border.value;

        // Emit the struct literal fragment expected by the generated `Theme` initializer.
        quote! {
            colors: Colors {
                text: #text,
                background: #background,
                accent: #accent,
                border: #border,
            }
        }
    }
}
