//! Token generation for the public `Tabler` enum produced by the macro.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Attribute;

use crate::{
    cache::load_icon,
    input::{IconDefinition, TablerInput},
    naming,
};

struct VariantDefinition {
    ident: Ident,
    icon_key: String,
    merged_name: String,
    svg_content: String,
}

/// Expands the parsed macro input into the final `Tabler` enum implementation.
pub(crate) fn expand_tabler_icon(input: TablerInput) -> TokenStream {
    let TablerInput {
        attrs,
        icons,
    } = input;
    let attributes = enum_attributes(&attrs);
    let variants = collect_variants(&icons);

    let variant_idents: Vec<_> = variants.iter().map(|variant| &variant.ident).collect();
    let as_str_match_arms: Vec<_> = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let svg_content = &variant.svg_content;

            quote! {
                Self::#ident => { #svg_content }
            }
        })
        .collect();

    let merged_name_arms: Vec<_> = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let merged_name = &variant.merged_name;

            quote! {
                Self::#ident => #merged_name
            }
        })
        .collect();

    let from_str_arms: Vec<_> = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let icon_key = &variant.icon_key;

            quote! {
                #icon_key => Ok(Self::#ident),
            }
        })
        .collect();

    let display_arms: Vec<_> = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let icon_key = &variant.icon_key;

            quote! {
                Self::#ident => write!(f, #icon_key)
            }
        })
        .collect();

    let leptos_methods = quote! {
        #[allow(unexpected_cfgs)]
        #[cfg(feature = "leptos")]
        /// Converts the icon SVG into a Leptos view.
        pub fn to_leptos(&self) -> ::leptos::prelude::AnyView {
            use ::leptos::prelude::*;

            match self {
                #(Self::#variant_idents => {
                    let svg = Self::#variant_idents.as_str();
                    view! { <div inner_html=svg /> }.into_any()
                })*
            }
        }
    };

    quote! {
        /// Icon enum with embedded SVG content
        ///
        /// Each variant represents an icon with its specific style (outline/filled)
        /// The SVG content is embedded in the binary at compile time.
        #attributes
        pub enum Tabler {
            #[default]
            #(#variant_idents),*
        }

        impl Tabler {
            /// Returns the embedded SVG content for the icon variant.
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str_match_arms),*
                }
            }

            /// Returns the icon display name without the style prefix.
            pub fn as_str_merget(&self) -> &str {
                match self {
                    #(#merged_name_arms),*
                }
            }

            /// Returns all generated icon variants in declaration order.
            pub fn all_icons() -> Vec<Self> {
                vec![#(Self::#variant_idents),*]
            }

            #leptos_methods
        }

        impl std::str::FromStr for Tabler {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let normalized = s.trim().to_lowercase();
                match normalized.as_str() {
                    #(#from_str_arms)*
                    _ => Err(format!("Unknown icon variant: {}", normalized))
                }
            }
        }

        impl std::fmt::Display for Tabler {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }
    }
}

/// Combines user-provided enum attributes with the crate's default derives.
fn enum_attributes(attrs: &[Attribute]) -> TokenStream {
    let default_derives = quote! {
        #[derive(Debug, Clone, PartialEq, Default)]
    };

    if attrs.is_empty() {
        default_derives
    } else {
        quote! {
            #(#attrs)*
            #default_derives
        }
    }
}

/// Builds the intermediate variant model consumed by the token generator.
fn collect_variants(icons: &[IconDefinition]) -> Vec<VariantDefinition> {
    let mut variants = Vec::new();

    for icon in icons {
        let display_name = naming::display_name(icon);
        let download_name = naming::download_name(icon);

        for style in &icon.styles {
            variants.push(VariantDefinition {
                ident: naming::variant_ident(style, &display_name),
                icon_key: naming::icon_key(style, &display_name),
                merged_name: display_name.clone(),
                svg_content: load_icon(style, &download_name),
            });
        }
    }

    variants
}
