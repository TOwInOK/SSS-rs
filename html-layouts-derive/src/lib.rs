use proc_macro::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct LayoutsInput {
    template_dir_path: LitStr,
    default_template_path: LitStr,
}

impl Parse for LayoutsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        let mut iter = content.into_iter();
        let template_dir_path = iter.next().unwrap();
        let default_template_path = iter.next().unwrap();
        Ok(LayoutsInput {
            template_dir_path,
            default_template_path,
        })
    }
}

#[proc_macro]
pub fn generate_layouts(input: TokenStream) -> TokenStream {
    let LayoutsInput {
        template_dir_path,
        default_template_path,
    } = parse_macro_input!(input as LayoutsInput);

    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let templates_dir = PathBuf::from(template_dir_path.value());
    let default_template = PathBuf::from(default_template_path.value());

    let templates_path = PathBuf::from(&cargo_dir)
        .join(&templates_dir)
        .canonicalize()
        .unwrap_or_else(|_| panic!("Failed to resolve templates path"));

    let default_template_path = PathBuf::from(&cargo_dir)
        .join(&default_template)
        .canonicalize()
        .unwrap_or_else(|_| panic!("Failed to resolve default layout template path"));

    // Проверка существования директории
    if !templates_path.exists() {
        panic!(
            "Templates directory not found at: {}",
            templates_path.display()
        );
    }

    impl_generate_layouts(templates_path, default_template_path).into()
}

fn impl_generate_layouts(
    layouts_dir: PathBuf,
    default_template_path: PathBuf,
) -> proc_macro2::TokenStream {
    let mut variants = Vec::new();
    let mut card_template_consts = Vec::new();
    let mut to_layout_matches = Vec::new();
    let mut display_matches = Vec::new();
    let mut from_str_matches = Vec::new();

    let default_template_path = default_template_path.to_str().unwrap();
    let default_template = std::fs::read_to_string(default_template_path).unwrap();

    if let Ok(entries) = std::fs::read_dir(&layouts_dir) {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                if let Some(name) = entry.path().file_name() {
                    if let Some(name_str) = name.to_str() {
                        let card_path = entry.path().join("card.html.tera");

                        // Skip hidden directories and check required templates
                        if name_str.starts_with('.') || !card_path.exists() {
                            continue;
                        }

                        let variant_name = syn::Ident::new(
                            &name_str.to_uppercase(),
                            proc_macro2::Span::mixed_site(),
                        );

                        let lower_variant_name = syn::LitStr::new(
                            &name_str.to_lowercase(),
                            proc_macro2::Span::call_site(),
                        );

                        let card_template_const_name = syn::Ident::new(
                            &format!("{}_CARD_TEMPLATE", name_str.to_uppercase()),
                            proc_macro2::Span::call_site(),
                        );

                        variants.push(variant_name.clone());

                        // Convert paths to strings for include_str!
                        let card_template_path = card_path.to_str().unwrap();

                        card_template_consts.push(quote! {
                            pub const #card_template_const_name: &str = include_str!(#card_template_path);
                        });

                        to_layout_matches.push(quote! {
                            HtmlLayouts::#variant_name => {
                                HtmlTeraRender::new(
                                    settings,
                                    theme,
                                    #card_template_const_name,
                                )
                            }
                        });

                        display_matches.push(quote! {
                            HtmlLayouts::#variant_name => write!(f, #name_str)
                        });

                        from_str_matches.push(quote! {
                            #lower_variant_name => Ok(Self::#variant_name),
                        });
                    }
                }
            }
        }
    }

    if variants.is_empty() {
        panic!("No layouts found in {}", layouts_dir.display());
    }

    quote! {
        use std::fmt;

        #(#card_template_consts)*

        #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
        #[derive(Debug, Deserialize, Serialize, Clone, clap::ValueEnum, PartialEq)]
        pub enum HtmlLayouts {
            #(#variants),*
        }


        impl Default for HtmlLayouts {
            fn default() -> Self {
                HtmlLayouts::UMBRELLA
            }
        }

        impl HtmlLayouts {
            pub fn render<'a>(
                &self,
                settings: &'a Settings,
                theme: &'static Theme,
            ) -> HtmlTeraRender<'a> {
                match self {
                    #(#to_layout_matches)*
                }
            }
            pub fn finalize<'a>(&self, settings: &'a Settings, theme: &'static Theme) -> HtmlTeraFinalize<HtmlTeraRender<'a>> {
                (self.render(settings, theme)).finalize(#default_template)
            }

            pub fn all_layouts() -> Vec<HtmlLayouts> {
                vec![
                    #(HtmlLayouts::#variants),*
                ]
            }
        }

        impl fmt::Display for HtmlLayouts {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    #(#display_matches),*
                }
            }
        }

        impl std::str::FromStr for HtmlLayouts {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<HtmlLayouts, String> {
                match s.to_lowercase().as_str() {
                    #(#from_str_matches)*
                    _ => Err(format!("'{}' is not a valid Layout", s))
                }
            }
        }
    }
}

// TODO: Подумать над тем как именно и что именно вытаскивать из Layout
