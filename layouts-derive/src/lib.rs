use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn generate_layouts(input: TokenStream) -> TokenStream {
    let base_dir = parse_macro_input!(input as LitStr).value();
    let full_path = format!(
        "{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()),
        base_dir
    );
    impl_generate_layouts(full_path).into()
}

fn impl_generate_layouts(layouts_dir: String) -> proc_macro2::TokenStream {
    let mut variants = Vec::new();
    let mut template_consts = Vec::new();
    let mut to_layout_matches = Vec::new();
    let mut display_matches = Vec::new();

    let templates_path = Path::new(&layouts_dir);

    if let Ok(entries) = std::fs::read_dir(templates_path) {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                if let Some(name) = entry.path().file_name() {
                    if let Some(name_str) = name.to_str() {
                        // Пропускаем служебные директории и файлы
                        if name_str.starts_with('.')
                            || !entry.path().join("card.html.tera").exists()
                        {
                            continue;
                        }

                        let variant_name = syn::Ident::new(
                            &name_str.to_uppercase(),
                            proc_macro2::Span::call_site(),
                        );
                        let template_const_name = syn::Ident::new(
                            &format!("{}_TERA_CARD_TEMPLATE", name_str.to_uppercase()),
                            proc_macro2::Span::call_site(),
                        );

                        variants.push(variant_name.clone());

                        let template_path = format!("{}/{}/card.html.tera", layouts_dir, name_str);
                        template_consts.push(quote! {
                            pub const #template_const_name: &str = include_str!(#template_path);
                        });

                        to_layout_matches.push(quote! {
                            Layouts::#variant_name => Box::new(HtmlTeraRender::new(
                                settings,
                                theme,
                                #template_const_name,
                                DEFAULT_TERA_LAYOUT,
                            ))
                        });

                        display_matches.push(quote! {
                            Layouts::#variant_name => write!(f, #name_str)
                        });
                    }
                }
            }
        }
    }

    // Проверяем, что нашли хотя бы один вариант
    if variants.is_empty() {
        panic!("No valid layout directories found in {}", layouts_dir);
    }

    quote! {
        use std::fmt;

        #(#template_consts)*

        #[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum)]
        pub enum Layouts {
            #[default]
            #(#variants),*
        }

        impl Layouts {
            pub fn to_layout<'a>(
                &self,
                settings: &'a Settings,
                theme: &'static Theme,
            ) -> Box<impl Finalize + 'a> {
                match self {
                    #(#to_layout_matches),*
                }
            }
        }

        impl fmt::Display for Layouts {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    #(#display_matches),*
                }
            }
        }
    }
}
