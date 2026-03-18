//! Root parser and code generator for the theme macro input tree.

use quote::quote;
use syn::{Ident, parse::Parse};

use crate::theme::ThemeLayout;

/// Parsed root node containing all theme declarations passed to the macro.
pub struct Node {
    pub vec: Vec<ThemeLayout>,
}

impl Parse for Node {
    /// Parses the complete macro input into a list of theme declarations.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Accumulate parsed themes in declaration order so generated output stays predictable.
        let mut vec = Vec::new();

        // Reject an empty invocation early with a dedicated error message.
        if input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "Expected theme items but got nothing",
            ));
        }

        // Keep parsing theme blocks until the macro input is fully consumed.
        while !input.is_empty() {
            vec.push(input.parse()?);
        }

        Ok(Node { vec })
    }
}

impl Node {
    /// Generates all Rust items required for the parsed set of themes.
    pub fn generate(&self) -> proc_macro2::TokenStream {
        // Build helper types and trait impls that operate on the generated themes.
        let implementations = self.generate_impl();
        // Render each parsed theme into its own static item.
        let themes = self.themes();

        // Return a single token stream containing both static theme values and helper API.
        quote! {
            #themes
            #implementations
        }
    }
}

impl Node {
    /// Collects theme names normalized to lowercase strings.
    fn labels(&self) -> Vec<String> {
        // Lowercase names are reused in string-based conversions such as `Display` and `FromStr`.
        self.vec
            .iter()
            .map(|x| x.name.to_string().to_lowercase())
            .collect()
    }

    /// Collects theme names converted into uppercase identifiers.
    fn labels_upper(&self) -> Vec<Ident> {
        // Uppercase identifiers are used for enum variants and generated static names.
        self.vec
            .iter()
            .map(|x| Ident::new(x.name.to_string().to_uppercase().as_str(), x.name.span()))
            .collect::<Vec<_>>()
    }

    /// Generates the base `Themes` enum covering every declared theme.
    fn generate_base(&self) -> proc_macro2::TokenStream {
        // Reuse the uppercase identifiers as enum variants in the generated API.
        let labels = self.labels_upper();
        quote! {
            /// Enumerates all themes declared in the macro input.
            #[allow(unexpected_cfgs)]
            #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
            #[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum, PartialEq)]
            #[allow(non_camel_case_types)]
            pub enum Themes {
                #[default]
                #(#labels),*
            }
        }
    }

    /// Generates shared helper methods for the `Themes` enum.
    fn base_impl() -> proc_macro2::TokenStream {
        // Each accessor goes through `From<&Themes>` so the generated logic stays centralized.
        quote! {
            /// Returns the font tuple associated with the selected theme.
            pub fn font(&self) -> (&'static str, &'static str) {
                let theme: &Theme = self.into();
                theme.font
            }

            /// Returns the color palette associated with the selected theme.
            pub fn colors(&self) -> &Colors {
                let theme: &Theme = self.into();
                &theme.colors
            }

            /// Returns the complete static theme definition.
            pub fn theme(&self) -> &Theme {
                let theme: &Theme = self.into();
                theme
            }
        }
    }

    /// Generates a helper that returns every available theme variant.
    fn generate_all_items(&self) -> proc_macro2::TokenStream {
        // Preserve declaration order so callers receive a deterministic list of variants.
        let labels = self.labels_upper();
        quote! {
            /// Returns all generated theme variants in declaration order.
            pub fn all_themes() -> Vec<Self> {
                vec![
                    #(Self::#labels),*
                ]
            }
        }
    }

    /// Generates the `Display` implementation for the `Themes` enum.
    fn generate_display(&self) -> proc_macro2::TokenStream {
        // Pair lowercase labels with their enum variants for human-readable formatting.
        let labels = self.labels();
        let upper = self.labels_upper();
        quote! {
            impl std::fmt::Display for Themes {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(Themes::#upper => write!(f, "{}", #labels),)*
                    }
                }
            }
        }
    }

    /// Generates conversions from theme variants into references to static themes.
    fn generate_from(&self) -> proc_macro2::TokenStream {
        // Reuse one matcher for owned and borrowed enum conversions to avoid duplication.
        let upper = self.labels_upper();
        let matcher = quote! {
            #(Themes::#upper => &#upper),*
        };
        quote! {
            impl From<Themes> for &'static Theme {
                fn from(value: Themes) -> &'static Theme {
                    match value {
                         #matcher
                    }
                }
            }

            impl From<&Themes> for &'static Theme {
                fn from(value: &Themes) -> &'static Theme {
                    match value {
                        #matcher
                    }
                }
            }
        }
    }

    /// Generates case-insensitive string parsing for the `Themes` enum.
    fn generate_from_str(&self) -> proc_macro2::TokenStream {
        // Build match arms using normalized lowercase names so parsing is case-insensitive.
        let uppers = self.labels_upper();
        let labels = self.labels();
        let matcher = quote! {
            #(#labels => Ok(Self::#uppers)),*
        };
        quote! {
            impl std::str::FromStr for Themes {
                type Err = String;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    // Normalize the input once before matching against known theme labels.
                    let s = s.to_lowercase();
                    match s.as_str() {
                        #matcher,
                        _ => Err(format!("Uncorrect theme: {}", s)),
                    }
                }
            }
        }
    }
}

impl Node {
    /// Generates static theme items for each parsed theme declaration.
    fn themes(&self) -> proc_macro2::TokenStream {
        // Render every parsed theme independently and concatenate the resulting items.
        let themes = self.vec.iter().map(|x| x.generate()).collect::<Vec<_>>();
        quote! {
            #(#themes)*
        }
    }
}

impl Node {
    /// Combines all generated helper fragments into the final `Themes` API.
    fn generate_impl(&self) -> proc_macro2::TokenStream {
        // Build each fragment separately to keep the generation pipeline easy to follow.
        let base = self.generate_base();
        let display = self.generate_display();
        let from = self.generate_from();
        let from_str = self.generate_from_str();
        let base_impl = Self::base_impl();
        let all_items = self.generate_all_items();

        // Stitch the fragments together into the final helper enum and trait impls.
        quote! {
            #base
            impl Themes {
                #base_impl
                #all_items
            }
            #from_str
            #from
            #display
        }
    }
}
