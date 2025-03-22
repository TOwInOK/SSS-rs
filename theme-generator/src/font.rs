use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

// Структура для шрифта
pub struct FontLayout {
    pub name: FontField,
    pub link: FontField,
}

pub struct FontField {
    pub name: Ident,
    _colon_token: Token![:],
    pub value: LitStr,
    _comma_token: Option<Token![,]>,
}

impl Parse for FontField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(FontField {
            name: input.parse()?,
            _colon_token: input.parse()?,
            value: input.parse()?,
            _comma_token: if input.peek(Token![,]) { Some(input.parse()?) } else { None },
        })
    }
}

impl Parse for FontLayout {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let font_token: Ident = input.parse()?;
        if font_token != "font" {
            return Err(input.error("expected 'font'"));
        }
        let content;
        syn::braced!(content in input);

        let name: FontField = content.parse()?;
        if name.name != "name" {
            return Err(content.error("expected name"));
        }

        let link: FontField = content.parse()?;
        if link.name != "link" {
            return Err(content.error("expected link"));
        }

        Ok(FontLayout { name, link })
    }
}
impl FontLayout {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name.value.value();
        let link = &self.link.value.value();
        quote! {
            font: (
                #name,
                #link,
            )
        }
    }
}
