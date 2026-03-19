//! Code generation for `HtmlLayouts` and its associated limitations.
//!
//! This module scans layout directories that contain card templates and limitation files, then
//! turns them into Rust token streams used by the `generate_layouts!` proc-macro.

use std::path::Path;

use parser::parse::parse;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use sss_core::LayoutLimitations;
use syn::{Error, Ident, LitStr};

use crate::{
    discovery::{ensure_exists, find_layout_directories, read_utf8_file, resolve_macro_path},
    types::Templates,
};

const TEMPLATE_CARD_NAME: &str = "card.html.tera";
const TEMPLATE_CARD_LIMITATIONS_NAME: &str = "card.limitations.ron";

/// Resolves the templates directory and generates the `HtmlLayouts` token stream from it.
pub fn generate_stream(path: &Path) -> Result<TokenStream, Error> {
    // Interpret the macro argument relative to the consuming crate before scanning the directory.
    let templates_path = resolve_macro_path(path, "templates directory")?;

    generate(&templates_path)
}

/// Builds all generated items related to concrete card layouts.
fn generate(layouts_dir: &Path) -> Result<proc_macro2::TokenStream, Error> {
    // Discover templates once so every generated fragment works from the same ordered layout set.
    let templates = find_templates(layouts_dir)?;

    // Discovery sorts layouts deterministically, so taking the first template yields a stable
    // `Default` implementation instead of depending on filesystem iteration order.
    let first_template = templates
        .first()
        .map(|template| &template.layout_ident)
        .ok_or_else(|| {
            Error::new(
                Span::call_site(),
                format!("No template layouts found in {}", layouts_dir.display()),
            )
        })?;

    // Assemble each generated concern separately to keep the emitted code easy to inspect.
    let variants = generates_variants(&templates);
    let layout_trait = generate_layout_trait(&templates);
    let limitations_trait = generate_limitations_trait(&templates);
    let display = generate_display_matches(&templates);
    let from_str = generate_from_str_matches(&templates);
    let impls = generate_impl(&templates);
    let const_limitations_names = generate_const_limitations_names(&templates);

    Ok(quote! {
        #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
        #[derive(Debug, Deserialize, Serialize, Clone, clap::ValueEnum, PartialEq)]
        #variants

        // Generated trait and utility impls are emitted next to the enum so the consuming crate
        // gets a complete layout API from a single macro invocation.
        #layout_trait

        #limitations_trait

        #display

        #from_str

        #impls

        impl Default for HtmlLayouts {
            fn default() -> Self {
                HtmlLayouts::#first_template
            }
        }

        #const_limitations_names
    })
}

/// Reads all card templates and their limitation files from discovered layout directories.
fn find_templates(templates: &Path) -> Result<Vec<Templates>, syn::Error> {
    find_layout_directories(templates, "template layout")?
        .into_iter()
        .map(|directory| {
            // Each template layout must provide both the card template and its declared limits.
            let template_path = directory.path.join(TEMPLATE_CARD_NAME);
            let limitations_path = directory.path.join(TEMPLATE_CARD_LIMITATIONS_NAME);

            ensure_exists(&template_path, "template file")?;
            ensure_exists(&limitations_path, "limitations file")?;

            // Read the template source verbatim because it will be embedded into the generated enum.
            let template = read_utf8_file(&template_path, "template file")?;

            // Parse the RON limitations file now so invalid layout metadata fails the compilation
            // at macro expansion time instead of later at runtime.
            let limitations = parse::<LayoutLimitations>(&limitations_path).map_err(|error| {
                Error::new(
                    Span::call_site(),
                    format!(
                        "Failed to parse limitations file '{}': {}",
                        limitations_path.display(),
                        error
                    ),
                )
            })?;
            let layout_ident = directory.layout_ident;

            // Materialize a dedicated constant name so the generated `limitations()` method can
            // refer to a readable top-level constant instead of repeating the entire literal.
            let const_limitations_name = limitations_const_ident(&layout_ident)?;

            Ok(Templates {
                layout_name: LitStr::new(&directory.layout_name, Span::call_site()),
                layout_ident,
                const_limitations_name,
                template,
                limitations,
            })
        })
        .collect()
}

/// Constructs the identifier used for the generated limitations constant of one layout.
fn limitations_const_ident(layout_ident: &Ident) -> Result<Ident, Error> {
    // Prefix the variant name so the generated constants occupy their own predictable namespace.
    let candidate = format!("TERA_HTML_TEMPLATE_LIMITATIONS_{}", layout_ident);

    syn::parse_str::<Ident>(&candidate).map_err(|_| {
        Error::new(
            Span::call_site(),
            format!("Failed to construct limitations constant identifier '{candidate}'"),
        )
    })
}

/// Generates the `HtmlLayouts` enum declaration from the discovered layout identifiers.
fn generates_variants(templates: &[Templates]) -> TokenStream {
    // Preserve the discovery order so all generated impls refer to the variants consistently.
    let templates_stream = templates
        .iter()
        .map(|t| &t.layout_ident)
        .collect::<Vec<_>>();
    quote! {
        pub enum HtmlLayouts {
            #(#templates_stream),*
        }
    }
}

/// Generates the body of `Layout::template()` for `HtmlLayouts`.
fn generate_layout_matches(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let template = &t.template;
            quote! {
                HtmlLayouts::#upper => {
                    // Embed the template source directly into the generated match arm so layout
                    // lookup stays purely static after compilation.
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

/// Generates the body of `Limitations::limitations()` for `HtmlLayouts`.
fn generate_layout_limitations_matches(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let const_name = &t.const_limitations_name;
            quote! {
                HtmlLayouts::#upper => {
                    // Each arm returns the layout-specific constant as an owned value so callers
                    // can treat all branches uniformly.
                    Some(std::borrow::Cow::Owned(#const_name))
                }
            }
        })
        .collect::<Vec<_>>();
    quote! {
        fn limitations(&self) -> Option<Cow<LayoutLimitations>> {
            match self {
                #(#templates_stream),*
            }
        }
    }
}

/// Generates the `Layout<String>` implementation for `HtmlLayouts`.
fn generate_layout_trait(templates: &[Templates]) -> TokenStream {
    // Reuse the generated match body so the trait impl stays focused on the outer structure.
    let template = generate_layout_matches(templates);
    quote! {
        impl Layout<String> for HtmlLayouts {
            #template
        }
    }
}

/// Generates the `Limitations` implementation for `HtmlLayouts`.
fn generate_limitations_trait(templates: &[Templates]) -> TokenStream {
    // Limitations are generated separately because they rely on additional top-level constants.
    let limitations = generate_layout_limitations_matches(templates);
    quote! {
        impl Limitations for HtmlLayouts {
            #limitations
        }
    }
}

/// Generates the `Display` implementation that exposes the normalized layout name.
fn generate_display_matches(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let name = &t.layout_name;
            quote! {
                // `Display` uses the lowercase normalized name so it matches the names accepted by
                // `FromStr` and external configuration.
                HtmlLayouts::#upper => write!(f, #name)
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl fmt::Display for HtmlLayouts {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    #(#templates_stream),*
                }
            }
        }
    }
}

/// Generates the `FromStr` implementation used to parse layout names from strings.
fn generate_from_str_matches(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let namer = &t.layout_name;
            let upper = &t.layout_ident;
            quote! {
                // Match against the normalized lowercase name so parsing remains case-insensitive.
                #namer => Ok(Self::#upper),
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl std::str::FromStr for HtmlLayouts {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<HtmlLayouts, String> {
                // Normalize the caller input before matching because discovery already normalized
                // the generated layout names.
                match s.to_lowercase().as_str() {
                    #(#templates_stream)*
                    _ => Err(format!("'{}' is not a valid Layout", s))
                }
            }
        }
    }
}

/// Generates the `HtmlLayouts::all()` helper that returns every generated variant.
fn generate_all_layouts(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            quote! {
                // Preserve discovery order so callers receive a deterministic list of layouts.
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

/// Generates inherent helper methods for `HtmlLayouts`.
fn generate_impl(templates: &[Templates]) -> TokenStream {
    // Keep inherent methods grouped in their own impl block to mirror handwritten Rust code.
    let all_layouts = generate_all_layouts(templates);
    quote! {
        impl HtmlLayouts {
            #all_layouts
        }
    }
}

/// Generates the top-level constants that store parsed layout limitations.
fn generate_const_limitations_names(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let const_name = &t.const_limitations_name;
            let limitations = layout_limitations_literal(&t.limitations);

            quote! {
                // Materialize the parsed limitations as a constant once so the generated enum impls
                // can reference them without repeating the entire struct literal in every match arm.
                const #const_name: sss_core::LayoutLimitations = #limitations;
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #(#templates_stream)*
    }
}

/// Converts a parsed `LayoutLimitations` value into a tokenized Rust struct literal.
fn layout_limitations_literal(limitations: &LayoutLimitations) -> TokenStream {
    // Destructure the value into plain primitives first because quoting simple values produces
    // clearer generated code than repeatedly indexing into nested fields inside `quote!`.
    let user = user_limitations_literal(limitations.user);
    let (specifications_count1, specifications_count2) = limitations.specifications_count;
    let about = option_usize_literal(limitations.about);
    let (repositories1, repositories2) = limitations.repositories;
    let socials = option_usize_literal(limitations.socials);
    let (skills_size, skills_data) = limitations.skills;
    let skill_skill_len = skills_data.skill_len;
    let skill_projects = option_pair_literal(skills_data.projects);
    let skill_since = skills_data.since;
    let skill_main = skills_data.main;
    let skill_repo_link = skills_data.repo_link;

    // The literal must mirror `sss_core::LayoutLimitations` exactly because the generated value is
    // compiled directly into the consuming crate as a `const` item.
    quote! {
        sss_core::LayoutLimitations {
            user: #user,
            specifications_count: (#specifications_count1, #specifications_count2),
            about: #about,
            repositories: (#repositories1, #repositories2),
            socials: #socials,
            skills: (
                #skills_size,
                sss_core::types::skill::SkillLimitation {
                    skill_len: #skill_skill_len,
                    projects: #skill_projects,
                    since: #skill_since,
                    main: #skill_main,
                    repo_link: #skill_repo_link,
                }
            ),
        }
    }
}

/// Converts optional nested user limitations into a tokenized Rust expression.
fn user_limitations_literal(user: Option<sss_core::types::user::UserLimitations>) -> TokenStream {
    match user {
        Some(user) => {
            // Copy the scalar fields out first so the generated literal stays straightforward.
            let name_len = user.name_len;
            let global_nickname_len = user.global_nickname_len;
            let global_nickname_pronounce_len = user.global_nickname_pronounce_len;
            let prevision_nicknames_max_count = user.prevision_nicknames_max_count;

            quote! {
                Some(sss_core::types::user::UserLimitations {
                    name_len: #name_len,
                    global_nickname_len: #global_nickname_len,
                    global_nickname_pronounce_len: #global_nickname_pronounce_len,
                    prevision_nicknames_max_count: #prevision_nicknames_max_count,
                })
            }
        }
        None => quote! { None },
    }
}

/// Converts an optional `usize` into a tokenized `Option<usize>` literal.
fn option_usize_literal(value: Option<usize>) -> TokenStream {
    match value {
        Some(value) => quote! { Some(#value) },
        None => quote! { None },
    }
}

/// Converts an optional pair into a tokenized `Option<(usize, usize)>` literal.
fn option_pair_literal(value: Option<(usize, usize)>) -> TokenStream {
    match value {
        Some((first, second)) => quote! { Some((#first, #second)) },
        None => quote! { None },
    }
}

#[cfg(test)]
mod tests {
    //! Tests for literal generation helpers used by the proc-macro.

    use sss_core::LayoutLimitations;

    use super::layout_limitations_literal;

    #[test]
    /// Verifies that missing user limitations remain `None` in the generated token stream.
    fn preserves_absent_user_limitations() {
        // The default value contains no nested user limitations, which should stay explicit in the
        // generated literal instead of producing an empty struct.
        let tokens = layout_limitations_literal(&LayoutLimitations::default()).to_string();

        assert!(tokens.contains("user : None"));
    }
}
