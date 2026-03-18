//! Parsing and normalization for the [`tabler_icon!`](crate::tabler_icon) input syntax.

use syn::{
    Attribute, Expr, Lit, Meta, Result, Token, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

#[derive(Debug)]
pub(crate) struct IconDefinition {
    /// Optional display name provided by `#[name = "..."]` next to the icon.
    pub(crate) display_name: Option<String>,
    /// Original icon identifier as it appears in the macro invocation.
    pub(crate) name: String,
    /// Requested icon style variants, for example `outline` or `filled`.
    pub(crate) styles: Vec<String>,
}

impl Parse for IconDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut display_name = None;
        let attrs = Attribute::parse_outer(input)?;

        for attr in attrs {
            if let Meta::NameValue(meta) = attr.meta
                && meta.path.is_ident("name")
                    && let Expr::Lit(expr_lit) = meta.value
                        && let Lit::Str(lit_str) = expr_lit.lit {
                            display_name = Some(lit_str.value());
                        }
        }

        let name = input.parse::<syn::Ident>()?.to_string();

        let mut styles = Vec::new();
        if input.peek(syn::token::Bracket) {
            let content;
            bracketed!(content in input);
            let style_list = content.parse_terminated(syn::Ident::parse, Token![,])?;
            styles = style_list
                .into_iter()
                .map(|ident| ident.to_string())
                .collect();
        }

        Ok(Self {
            display_name,
            name,
            styles,
        })
    }
}

#[derive(Debug)]
pub(crate) struct TablerInput {
    /// Attributes applied to the generated enum, with top-level `#[name = "..."]`
    /// handled during normalization for backward compatibility.
    pub(crate) attrs: Vec<Attribute>,
    /// Parsed list of icon declarations passed to the macro.
    pub(crate) icons: Vec<IconDefinition>,
}

impl Parse for TablerInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = Attribute::parse_outer(input).unwrap_or_default();
        let icons = Punctuated::<IconDefinition, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            icons,
        })
    }
}

impl TablerInput {
    /// Applies legacy normalization rules after parsing the macro input.
    ///
    /// In particular, a top-level `#[name = "..."]` still renames the first icon,
    /// preserving the existing public behavior of the macro.
    pub(crate) fn normalize(mut self) -> Self {
        apply_first_icon_name_attribute(&mut self.attrs, &mut self.icons);
        self
    }
}

/// Preserves the crate's legacy behavior where a top-level `#[name = "..."]`
/// targets the first icon entry instead of becoming an enum attribute.
fn apply_first_icon_name_attribute(
    attrs: &mut Vec<Attribute>,
    icons: &mut [IconDefinition],
) {
    if icons.is_empty() {
        panic!("Icon set is empty - must define at least one icon");
    }

    if let Some(name_index) = attrs
        .iter()
        .position(|attr| matches!(&attr.meta, Meta::NameValue(meta) if meta.path.is_ident("name")))
    {
        let name_attr = attrs.remove(name_index);
        let name_str = match name_attr.meta {
            Meta::NameValue(name_value) => match name_value.value {
                Expr::Lit(lit) => match lit.lit {
                    Lit::Str(lit_str) => lit_str.value(),
                    _ => panic!("Invalid literal type in name attribute - expected string literal"),
                },
                _ => panic!("Expected literal expression in name attribute"),
            },
            _ => panic!("Unexpected meta format for name attribute"),
        };

        icons
            .first_mut()
            .expect("Failed to get first icon")
            .display_name = Some(name_str);
    }
}
