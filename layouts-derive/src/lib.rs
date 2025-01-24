use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

/// Macro that generates layout enums and implementations based on directory structure
#[proc_macro]
pub fn generate_layouts(input: TokenStream) -> TokenStream {
    // Get the base directory path from macro input
    let base_dir = parse_macro_input!(input as LitStr).value();
    // Construct full path by combining CARGO_MANIFEST_DIR with base_dir
    let full_path = format!(
        "{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()),
        base_dir
    );
    impl_generate_layouts(full_path).into()
}

/// Core implementation of layout generation
fn impl_generate_layouts(layouts_dir: String) -> proc_macro2::TokenStream {
    // Vectors to store generated code components
    let mut variants = Vec::new(); // Enum variants
    let mut template_consts = Vec::new(); // Template constants
    let mut to_layout_matches = Vec::new(); // Match arms for to_layout method
    let mut display_matches = Vec::new(); // Match arms for Display implementation

    let templates_path = Path::new(&layouts_dir);

    if let Ok(entries) = std::fs::read_dir(templates_path) {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                if let Some(name) = entry.path().file_name() {
                    if let Some(name_str) = name.to_str() {
                        // Skip hidden directories and directories without card.html.tera
                        if name_str.starts_with('.')
                            || !entry.path().join("card.html.tera").exists()
                        {
                            continue;
                        }

                        // Create identifier for enum variant
                        let variant_name = syn::Ident::new(
                            &name_str.to_uppercase(),
                            proc_macro2::Span::call_site(),
                        );
                        // Create identifier for template constant
                        let template_const_name = syn::Ident::new(
                            &format!("{}_TERA_CARD_TEMPLATE", name_str.to_uppercase()),
                            proc_macro2::Span::call_site(),
                        );

                        variants.push(variant_name.clone());

                        // Generate template constant with included template file
                        let template_path = format!("{}/{}/card.html.tera", layouts_dir, name_str);
                        template_consts.push(quote! {
                            pub const #template_const_name: &str = include_str!(#template_path);
                        });

                        // Generate match arm for to_layout method
                        to_layout_matches.push(quote! {
                            Layouts::#variant_name => Box::new(HtmlTeraRender::new(
                                settings,
                                theme,
                                #template_const_name,
                                DEFAULT_TERA_LAYOUT,
                            ))
                        });

                        // Generate match arm for Display implementation
                        display_matches.push(quote! {
                            Layouts::#variant_name => write!(f, #name_str)
                        });
                    }
                }
            }
        }
    }

    // Ensure at least one layout was found
    if variants.is_empty() {
        panic!("No valid layout directories found in {}", layouts_dir);
    }

    // Generate final code using quote
    quote! {
        use std::fmt;

        #(#template_consts)*

        // Generate Layouts enum with derived traits
        #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
        #[derive(Debug, Deserialize, Serialize, Clone, clap::ValueEnum, PartialEq)]
        pub enum Layouts {
            #(#variants),*
        }
        impl Default for Layouts {
            fn default() -> Self {
                Layouts::UMBRELLA
            }
        }

        impl Layouts {
            // Method to create layout renderer based on selected variant
            pub fn to_layout<'a>(
                &self,
                settings: &'a Settings,
                theme: &'static Theme,
            ) -> Box<impl Finalise + 'a> {
                match self {
                    #(#to_layout_matches),*
                }
            }
            // Method to create vector with all [Layouts]
            pub fn all_layouts() -> Vec<Layouts> {
                vec![
                    #(Layouts::#variants),*
                ]
            }
        }

        // Display implementation for Layouts enum
        impl fmt::Display for Layouts {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    #(#display_matches),*
                }
            }
        }
    }
}
