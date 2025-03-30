use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

use quote::quote;

use proc_macro2::{Span, TokenStream};
use syn::Error;

use crate::{generate::check_on_exist, types::DefaultLayout};

const TEMPLATE_DEFAULT_LAYOUT_NAME: &str = "default_layout.html.tera";

pub fn generate_default_layout_stream(path: &Path) -> Result<TokenStream, Error> {
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let templates_path = PathBuf::from(&cargo_dir)
        .join(path)
        .canonicalize()
        .map_err(|e| {
            syn::Error::new(
                Span::call_site(),
                format!("Failed to resolve templates path: {}", e),
            )
        })?;

    check_on_exist(&templates_path)?;

    generate(&templates_path)
}

fn generate(path: &Path) -> Result<TokenStream, Error> {
    let templates = find_templates(path)?;
    let variants = generates_variants(&templates);
    let layout_trait = generate_layout_trait(&templates);
    let display = generate_display_matches(&templates);
    let from_str = generate_from_str_matches(&templates);
    let impls = generate_impl(&templates);
    let limitations_trait = generate_layout_limitations_matches();

    Ok(quote! {
        #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
        #[derive(Debug, Deserialize, Serialize, Clone, clap::ValueEnum, PartialEq)]
        #variants

        #layout_trait
        #limitations_trait
        #display
        #from_str
        #impls
    })
}

/// Parse templates
fn find_templates(templates: &Path) -> Result<Vec<DefaultLayout>, syn::Error> {
    let mut results = Vec::new();

    let entries = read_dir(templates).map_err(|e| {
        Error::new(
            Span::call_site(),
            format!("Failed to read template directory: {}", e),
        )
    })?;

    for entry in entries.filter_map(Result::ok) {
        if entry.path().is_dir() {
            if let Some(layout_name_os_str) = entry.file_name().to_str() {
                let layout_name = layout_name_os_str.to_string();
                let layout_name_upper = layout_name.to_uppercase();
                let layout_name_lower = layout_name.to_lowercase();
                let template_path = entry.path().join(TEMPLATE_DEFAULT_LAYOUT_NAME);

                check_on_exist(&template_path)?;

                let template = read_to_string(&template_path)
                    .map_err(|e| {
                        Error::new(
                            Span::call_site(),
                            format!(
                                "Failed to read template file {}: {}",
                                template_path.display(),
                                e
                            ),
                        )
                    })?
                    .leak();

                results.push(DefaultLayout {
                    layout_name: syn::LitStr::new(
                        &layout_name_lower,
                        proc_macro2::Span::mixed_site(),
                    ),
                    layout_ident: syn::Ident::new(
                        &layout_name_upper,
                        proc_macro2::Span::mixed_site(),
                    ),
                    template,
                });
            }
        }
    }

    Ok(results)
}

/// Generate variants for [HtmlLayouts]
fn generates_variants(layouts: &[DefaultLayout]) -> TokenStream {
    let templates_stream = layouts.iter().map(|t| &t.layout_ident).collect::<Vec<_>>();
    quote! {
        pub enum DefaultTemplates {
            #(#templates_stream),*
        }
    }
}

/// Generate template for [Layout]
fn generate_layout_matches(layouts: &[DefaultLayout]) -> TokenStream {
    let templates_stream = layouts
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let template = t.template;
            quote! {
                DefaultTemplates::#upper => {
                    Cow::Owned(#template.to_string())
                }
            }
        })
        .collect::<Vec<_>>();
    quote! {
        fn template(&self) -> Cow<String> {
            match self {
                #(#templates_stream),*
            }
        }
    }
}

// Generate [Layout] trait
fn generate_layout_trait(templates: &[DefaultLayout]) -> TokenStream {
    let template = generate_layout_matches(templates);
    quote! {
        impl Layout<String> for DefaultTemplates {
            #template
        }
    }
}

/// Generate [Limitations] trait
fn generate_layout_limitations_matches() -> TokenStream {
    quote! {
        impl Limitations for DefaultTemplates {
            fn limitations(&self) -> Option<Cow<LayoutLimitations>> {
                match self {
                    _ => None
                }
            }
        }
    }
}

/// Generate [Display]
fn generate_display_matches(templates: &[DefaultLayout]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let name = &t.layout_name;
            quote! {
                DefaultTemplates::#upper => write!(f, #name)
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl fmt::Display for DefaultTemplates {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    #(#templates_stream),*
                }
            }
        }
    }
}

/// Generate [FromStr]
fn generate_from_str_matches(templates: &[DefaultLayout]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let namer = &t.layout_name;
            let upper = &t.layout_ident;
            quote! {
                #namer => Ok(Self::#upper),
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl std::str::FromStr for DefaultTemplates {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<DefaultTemplates, String> {
                match s.to_lowercase().as_str() {
                    #(#templates_stream)*
                    _ => Err(format!("'{}' is not a valid Layout", s))
                }
            }
        }
    }
}

/// Generate all layouts
fn generate_all_layouts(templates: &[DefaultLayout]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            quote! {
                Self::#upper,
            }
        })
        .collect::<Vec<_>>();
    quote! {
       pub fn all() -> Vec<Self> {
            vec![#(#templates_stream)*]
        }
    }
}

/// Return all variants in [DefaultLayout]
fn generate_impl(templates: &[DefaultLayout]) -> TokenStream {
    let all_layouts = generate_all_layouts(templates);
    quote! {
        impl DefaultTemplates {
            #all_layouts
        }
    }
}
