use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

pub struct ColorsLayout {
    pub text: ColorField,
    pub background: ColorField,
    pub accent: ColorField,
    pub border: ColorField,
}

pub struct ColorField {
    pub name: Ident,
    _separator: Token![:],
    pub value: LitStr,
    _after: Option<Token![,]>,
}

impl Parse for ColorField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let separator = input.parse()?;
        let value = input.parse()?;
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
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let colors_token: Ident = input.parse()?;

        // Проверка, что токен действительно "colors"
        if colors_token != "colors" {
            return Err(input.error("expected 'colors'"));
        }

        if input.is_empty() {
            return Err(input.error("expected text"));
        }

        let content;
        syn::braced!(content in input);

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

        // Проверка, что внутри скобок больше ничего нет
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
    pub fn generate(&self) -> TokenStream {
        let text = &self.text.value;
        let background = &self.background.value;
        let accent = &self.accent.value;
        let border = &self.border.value;

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
