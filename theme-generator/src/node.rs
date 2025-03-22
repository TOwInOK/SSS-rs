use quote::quote;
use syn::{Ident, parse::Parse};

use crate::theme::ThemeLayout;

pub struct Node {
    pub vec: Vec<ThemeLayout>,
}
impl Parse for Node {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut vec = Vec::new();

        if input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "Expected theme items but got nothing",
            ));
        }

        while !input.is_empty() {
            vec.push(input.parse()?);
        }

        Ok(Node { vec })
    }
}
// Public
impl Node {
    /// Generate impl block for [Theme]
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let implementations = self.generate_impl();
        let themes = self.themes();
        quote! {
            #themes
            #implementations
        }
    }
}

// Local
impl Node {
    /// Vec of labels
    fn labels(&self) -> Vec<String> {
        self.vec
            .iter()
            .map(|x| x.name.to_string().to_lowercase())
            .collect()
    }
    /// Vec of labels in uppercase
    fn labels_upper(&self) -> Vec<Ident> {
        self.vec
            .iter()
            .map(|x| Ident::new(x.name.to_string().to_uppercase().as_str(), x.name.span()))
            .collect::<Vec<_>>()
    }

    /// Generate enum of fields for [Themes]
    fn generate_base(&self) -> proc_macro2::TokenStream {
        let labels = self.labels_upper();
        quote! {
            #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
            #[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum, PartialEq)]
            #[allow(non_camel_case_types)]
            pub enum Themes {
                #[default]
                #(#labels),*
            }
        }
    }

    /// Base implementation for [Themes]
    fn base_impl() -> proc_macro2::TokenStream {
        quote! {
            pub fn font(&self) -> (&'static str, &'static str) {
                let theme: &Theme = self.into();
                theme.font
            }
            pub fn colors(&self) -> &Colors {
                let theme: &Theme = self.into();
                &theme.colors
            }
            pub fn theme(&self) -> &Theme {
                let theme: &Theme = self.into();
                theme
            }
        }
    }

    /// Generate vector with all items in [Themes]
    fn generate_all_items(&self) -> proc_macro2::TokenStream {
        let labels = self.labels_upper();
        quote! {
            pub fn all_themes() -> Vec<Self> {
                vec![
                    #(Self::#labels),*
                ]
            }
        }
    }

    /// Generate display for [Themes]
    fn generate_display(&self) -> proc_macro2::TokenStream {
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

    /// Generate from block for [Themes]
    fn generate_from(&self) -> proc_macro2::TokenStream {
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

    /// Generate [FromStr] block for [Themes]
    fn generate_from_str(&self) -> proc_macro2::TokenStream {
        let uppers = self.labels_upper();
        let labels = self.labels();
        let matcher = quote! {
            #(#labels => Ok(Self::#uppers)),*
        };
        quote! {
            impl std::str::FromStr for Themes {
                type Err = String;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
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

// Theme builder
impl Node {
    fn themes(&self) -> proc_macro2::TokenStream {
        let themes = self.vec.iter().map(|x| x.generate()).collect::<Vec<_>>();
        quote! {
            #(#themes)*
        }
    }
}

// Impl generator
impl Node {
    /// Generate impl block for [Theme]
    fn generate_impl(&self) -> proc_macro2::TokenStream {
        let base = self.generate_base();
        let display = self.generate_display();
        let from = self.generate_from();
        let from_str = self.generate_from_str();
        let base_impl = Self::base_impl();
        let all_items = self.generate_all_items();
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
