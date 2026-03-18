//! Code generation for `DefaultTemplates`, the outer wrapper layouts used by HTML rendering.
//!
//! Default layouts only wrap already rendered card HTML, so they generate template-related code but
//! intentionally do not produce per-layout limitation metadata.

use std::path::Path;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Error;

use crate::{
    discovery::{ensure_exists, find_layout_directories, read_utf8_file, resolve_macro_path},
    types::DefaultLayout,
};

const TEMPLATE_DEFAULT_LAYOUT_NAME: &str = "default_layout.html.tera";

/// Resolves the default-layout directory and generates the `DefaultTemplates` token stream.
pub fn generate_default_layout_stream(path: &Path) -> Result<TokenStream, Error> {
    // Interpret the macro argument relative to the consuming crate before scanning the directory.
    let templates_path = resolve_macro_path(path, "default layout directory")?;

    generate(&templates_path)
}

/// Builds all generated items related to outer default layouts.
fn generate(path: &Path) -> Result<TokenStream, Error> {
    // Discover the available layouts first so every generated impl uses the same deterministic set.
    let templates = find_templates(path)?;

    // Assemble the emitted code in small pieces for readability and easier future maintenance.
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

/// Reads all default layout templates from discovered layout directories.
fn find_templates(templates: &Path) -> Result<Vec<DefaultLayout>, syn::Error> {
    find_layout_directories(templates, "default layout")?
        .into_iter()
        .map(|directory| {
            // Default layouts only need their wrapper template file.
            let template_path = directory.path.join(TEMPLATE_DEFAULT_LAYOUT_NAME);

            ensure_exists(&template_path, "default layout template file")?;

            Ok(DefaultLayout {
                layout_name: syn::LitStr::new(&directory.layout_name, Span::call_site()),
                layout_ident: directory.layout_ident,
                // Read the template verbatim because it will be embedded directly into generated
                // match arms.
                template: read_utf8_file(&template_path, "default layout template file")?,
            })
        })
        .collect()
}

/// Generates the `DefaultTemplates` enum declaration.
fn generates_variants(layouts: &[DefaultLayout]) -> TokenStream {
    // Preserve discovery order so every generated impl refers to the same variant ordering.
    let templates_stream = layouts.iter().map(|t| &t.layout_ident).collect::<Vec<_>>();
    quote! {
        pub enum DefaultTemplates {
            #(#templates_stream),*
        }
    }
}

/// Generates the body of `Layout::template()` for `DefaultTemplates`.
fn generate_layout_matches(layouts: &[DefaultLayout]) -> TokenStream {
    let templates_stream = layouts
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let template = &t.template;
            quote! {
                DefaultTemplates::#upper => {
                    // Embed the wrapper template source directly into the generated enum branch.
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

/// Generates the `Layout<String>` implementation for `DefaultTemplates`.
fn generate_layout_trait(templates: &[DefaultLayout]) -> TokenStream {
    // Reuse the generated match body to keep the impl focused on the trait shell.
    let template = generate_layout_matches(templates);
    quote! {
        impl Layout<String> for DefaultTemplates {
            #template
        }
    }
}

/// Generates the `Limitations` implementation for `DefaultTemplates`.
fn generate_layout_limitations_matches() -> TokenStream {
    quote! {
        impl Limitations for DefaultTemplates {
            fn limitations(&self) -> Option<Cow<LayoutLimitations>> {
                // Default layouts wrap the rendered card and therefore do not impose card field
                // limitations of their own.
                None
            }
        }
    }
}

/// Generates the `Display` implementation for `DefaultTemplates`.
fn generate_display_matches(templates: &[DefaultLayout]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let name = &t.layout_name;
            quote! {
                // Use the normalized lowercase name so display output matches string parsing.
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

/// Generates the `FromStr` implementation for `DefaultTemplates`.
fn generate_from_str_matches(templates: &[DefaultLayout]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let namer = &t.layout_name;
            let upper = &t.layout_ident;
            quote! {
                // Accept the normalized lowercase name produced during discovery.
                #namer => Ok(Self::#upper),
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl std::str::FromStr for DefaultTemplates {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<DefaultTemplates, String> {
                // Normalize user input before matching so lookups stay case-insensitive.
                match s.to_lowercase().as_str() {
                    #(#templates_stream)*
                    _ => Err(format!("'{}' is not a valid Layout", s))
                }
            }
        }
    }
}

/// Generates the `DefaultTemplates::all()` helper that returns every generated variant.
fn generate_all_layouts(templates: &[DefaultLayout]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            quote! {
                // Preserve discovery order so callers get a stable list across builds.
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

/// Generates inherent helper methods for `DefaultTemplates`.
fn generate_impl(templates: &[DefaultLayout]) -> TokenStream {
    // Keep inherent helpers grouped in one impl block, matching the structure used for
    // `HtmlLayouts`.
    let all_layouts = generate_all_layouts(templates);
    quote! {
        impl DefaultTemplates {
            #all_layouts
        }
    }
}
