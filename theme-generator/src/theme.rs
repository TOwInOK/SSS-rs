use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Token, parse::Parse};

use crate::{colors::ColorsLayout, font::FontLayout};

pub struct ThemeLayout {
    pub name: Ident,
    pub colors: ColorsLayout,
    pub font: FontLayout,
}

impl Parse for ThemeLayout {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Парсим имя темы
        let name = input.parse()?;

        // Парсим содержимое в фигурных скобках
        let content;
        syn::braced!(content in input);

        // Парсим секцию colors
        let colors = content.parse()?;

        // Проверяем и парсим запятую
        if !content.peek(Token![,]) {
            return Err(syn::Error::new(
                content.span(),
                "Expected comma after colors section",
            ));
        }
        content.parse::<Token![,]>()?;

        // Парсим секцию font
        let font = content.parse()?;

        // Проверяем, что больше нет неожиданного содержимого
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
    pub fn generate(&self) -> TokenStream {
        let name = syn::Ident::new(&self.name.to_string().to_uppercase(), self.name.span());
        let colors = &self.colors.generate();
        let font = &self.font.generate();
        quote! {
            pub static #name: Theme = Theme {
                #colors,
                #font,
            };
        }
    }
}
