use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

use parser::parse::parse;
use proc_macro2::{Span, TokenStream};
use sss_core::LayoutLimitations;
use syn::{Error, Ident, LitStr};

use crate::types::Templates;
use quote::quote;

const TEMPLATE_CARD_NAME: &str = "card.html.tera";
const TEMPLATE_CARD_LIMITATIONS_NAME: &str = "card.limitations.ron";

pub fn generate_stream(path: &Path) -> Result<TokenStream, Error> {
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

    generate(templates_path)
}

pub fn check_on_exist(path: &Path) -> Result<(), Error> {
    if !path.exists() {
        return Err(Error::new(
            Span::call_site(),
            format!("Failure found path: {}", path.display()),
        ));
    }
    Ok(())
}

fn generate(layouts_dir: PathBuf) -> Result<proc_macro2::TokenStream, Error> {
    let templates = find_templates(&layouts_dir)?;
    let first_template = &templates
        .first()
        .expect("fount zero templates")
        .layout_ident;
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

/// Parse templates
fn find_templates(templates: &Path) -> Result<Vec<Templates>, syn::Error> {
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
                let const_limitations_name =
                    format!("TERA_HTML_TEMPLATE_LIMITATIONS_{}", layout_name_upper);
                let template_path = entry.path().join(TEMPLATE_CARD_NAME);
                let limitations_path = entry.path().join(TEMPLATE_CARD_LIMITATIONS_NAME);

                check_on_exist(&template_path)?;
                check_on_exist(&limitations_path)?;

                let template = read_to_string(&template_path).map_err(|e| {
                    Error::new(
                        Span::call_site(),
                        format!(
                            "Failed to read template file {}: {}",
                            template_path.display(),
                            e
                        ),
                    )
                })?;

                let limitations = parse::<LayoutLimitations>(&limitations_path).map_err(|e| {
                    Error::new(
                        Span::call_site(),
                        format!(
                            "Failed to parse limitations file {}: {}",
                            limitations_path.display(),
                            e
                        ),
                    )
                })?;

                results.push(Templates {
                    layout_name: LitStr::new(&layout_name_lower, Span::call_site()),
                    layout_ident: Ident::new(&layout_name_upper, Span::call_site()),
                    const_limitations_name: Ident::new(&const_limitations_name, Span::call_site()),
                    template: template.leak(),
                    limitations,
                });
            }
        }
    }

    Ok(results)
}

/// Generate variants for [HtmlLayouts]
fn generates_variants(templates: &[Templates]) -> TokenStream {
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

/// Generate template for [Layout]
fn generate_layout_matches(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let template = t.template;
            quote! {
                HtmlLayouts::#upper => {
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

/// Generate limitations for [Layout]
fn generate_layout_limitations_matches(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let const_name = &t.const_limitations_name;
            quote! {
                HtmlLayouts::#upper => {
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

/// Generate [Layout] trait
fn generate_layout_trait(templates: &[Templates]) -> TokenStream {
    let template = generate_layout_matches(templates);
    quote! {
        impl Layout<String> for HtmlLayouts {
            #template
        }
    }
}

/// Generate [Layout] trait
fn generate_limitations_trait(templates: &[Templates]) -> TokenStream {
    let limitations = generate_layout_limitations_matches(templates);
    quote! {
        impl Limitations for HtmlLayouts {
            #limitations
        }
    }
}

/// Generate [Display]
fn generate_display_matches(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let upper = &t.layout_ident;
            let name = &t.layout_name;
            quote! {
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

/// Generate [FromStr]
fn generate_from_str_matches(templates: &[Templates]) -> TokenStream {
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
        impl std::str::FromStr for HtmlLayouts {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<HtmlLayouts, String> {
                match s.to_lowercase().as_str() {
                    #(#templates_stream)*
                    _ => Err(format!("'{}' is not a valid Layout", s))
                }
            }
        }
    }
}

/// Generate all layouts
fn generate_all_layouts(templates: &[Templates]) -> TokenStream {
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

/// Return all variants in [HtmlLayouts]
fn generate_impl(templates: &[Templates]) -> TokenStream {
    let all_layouts = generate_all_layouts(templates);
    quote! {
        impl HtmlLayouts {
            #all_layouts
        }
    }
}

fn generate_const_limitations_names(templates: &[Templates]) -> TokenStream {
    let templates_stream = templates
        .iter()
        .map(|t| {
            let const_name = &t.const_limitations_name;
            let limitations = &t.limitations;

            // Достаем поля из limitations структуры
            let user = &limitations.user;
            let (specifications_count1, specifications_count2) = &limitations.specifications_count;
            let about = match limitations.about {
                Some(some) => quote! {Some(#some)},
                None => quote! {None},
            };
            let (repositories1, repositories2) = &limitations.repositories;
            let socials = match limitations.socials {
                Some(some) => quote! {Some(#some)},
                None => quote! {None},
            };
            let (skills_size, skills_data) = &limitations.skills;

            // Достаем поля из вложенной структуры UserLimitations
            let user_name_len = match user {
                Some(user) => user.name_len,
                None => 0,
            };
            let user_global_nickname_len = match user {
                Some(user) => user.global_nickname_len,
                None => 0,
            };
            let user_global_nickname_pronounce_len = match user {
                Some(user) => user.global_nickname_pronounce_len,
                None => 0,
            };
            let user_prevision_nicknames_max_count = match user {
                Some(user) => user.prevision_nicknames_max_count,
                None => 0,
            };

            // Достаем поля из вложенной структуры SkillLimitations
            let skill_skill_len = skills_data.skill_len;
            let skill_projects = &skills_data.projects;

            // Manually create the TokenStream for the Option<(usize, usize)> field
            let skill_projects_tokens = match skill_projects {
                Some((p1, p2)) => {
                    // p1 and p2 are &usize, quote! can handle them
                    quote! { Some((#p1, #p2)) }
                }
                None => {
                    quote! { None }
                }
            };
            let skill_since = skills_data.since;
            let skill_main = skills_data.main;
            let skill_repo_link = skills_data.repo_link;

            quote! {
                const #const_name: sss_core::LayoutLimitations =
                sss_core::LayoutLimitations {
                    user: Some(sss_core::types::user::UserLimitations {
                        name_len: #user_name_len,
                        global_nickname_len: #user_global_nickname_len,
                        global_nickname_pronounce_len: #user_global_nickname_pronounce_len,
                        prevision_nicknames_max_count: #user_prevision_nicknames_max_count,
                    }),
                    specifications_count: (#specifications_count1, #specifications_count2),
                    about: #about,
                    repositories: (#repositories1, #repositories2),
                    socials: #socials,
                    skills: (#skills_size, sss_core::types::skill::SkillLimitation {
                        skill_len: #skill_skill_len,
                        projects: #skill_projects_tokens,
                        since: #skill_since,
                        main: #skill_main,
                        repo_link: #skill_repo_link,
                    }),
                };
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #(#templates_stream)*
    }
}
