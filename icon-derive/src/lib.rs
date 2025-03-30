//! Proc macro to create icon enums with embedded SVG content
//!
//! This macro downloads SVG icons during compilation and embeds them in the binary.
//! Currently supports Tabler icons from unpkg.com.
//!
//! To find the name of the icon use [tabler](https://tabler.io/icons)
//!
//! all cached in target/icon_cache for 30 days
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use tabler_icon_definer::tabler_icon;
//!
//! tabler_icon!(
//!     brand_github[outline, filled],
//!     user[outline]
//! );
//! // Generate
//! // - Tabler::OUTLINE_BRAND_GITHUB
//! // - Tabler::FILLED_BRAND_GITHUB
//! // - Tabler::OUTLINE_USER
//! ```
//!
//! With custom name and attributes:
//! ```rust
//! use tabler_icon_definer::tabler_icon;
//!
//! tabler_icon!(
//!     #[derive(serde::Serialize)]
//!     #[name = "github"]
//!     brand_github[outline, filled],
//!     user[outline]
//! );
//! // Generate
//! // - Tabler::OUTLINE_GITHUB
//! // - Tabler::FILLED_GITHUB
//! // - Tabler::OUTLINE_USER
//! ```
//!
//! Using generated enum:
//! ```rust
//! # use tabler_icon_definer::tabler_icon;
//! # tabler_icon!(#[name = "github"] brand_github[outline, filled]);
//! let icon = Tabler::OUTLINE_GITHUB;
//! let svg = icon.as_str(); // Get SVG content
//! let name = icon.as_str_merget(); // Get icon name without style prefix
//! let full_name = icon.to_string(); // Get icon name with style prefix
//! ```
//!
//! # Limitations
//! - first item is default item

use std::{
    env, fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, Expr, Lit, Meta, Result, Token, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

#[derive(Debug)]
struct IconInner {
    display_name: Option<String>,
    name: String,
    styles: Vec<String>,
}

impl Parse for IconInner {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut display_name = None;
        let attrs = Attribute::parse_outer(input)?;
        for attr in attrs {
            if let Meta::NameValue(meta) = attr.meta {
                if meta.path.is_ident("name") {
                    if let Expr::Lit(expr_lit) = meta.value {
                        if let Lit::Str(lit_str) = expr_lit.lit {
                            display_name = Some(lit_str.value());
                        }
                    }
                }
            }
        }

        let name = input.parse::<syn::Ident>()?.to_string();

        let mut styles = Vec::new();
        if input.peek(syn::token::Bracket) {
            let content;
            bracketed!(content in input);
            let style_list = content.parse_terminated(syn::Ident::parse, Token![,])?;
            styles = style_list
                .into_iter()
                .map(|ident| ident.to_string())
                .collect();
        }
        Ok(IconInner {
            display_name,
            name,
            styles,
        })
    }
}
#[derive(Debug)]
struct IconItem {
    icons: Vec<IconInner>,
}

impl Parse for IconItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let content = Punctuated::<IconInner, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            icons: content.into_iter().collect(),
        })
    }
}
#[derive(Debug)]
struct TablerEnter {
    attrs: Vec<Attribute>,
    icons: IconItem,
}

impl Parse for TablerEnter {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = Attribute::parse_outer(input).unwrap_or_default();
        // Парсим список иконок
        // panic!("{:#?}", attrs);
        let icons: IconItem = input.parse()?;
        Ok(TablerEnter {
            attrs,
            icons,
        })
    }
}

/// Proc macro to create icon enums with embedded SVG content
///
/// This macro downloads SVG icons during compilation and embeds them in the binary.
/// Currently supports Tabler icons from unpkg.com.
///
/// To find the name of the icon use [tabler](https://tabler.io/icons)
///
/// All cached in target/icon_cache for 30 days
///
/// # Examples
///
/// Basic usage:
/// ```ignore
/// use tabler_icon_definer::tabler_icon;
///
/// tabler_icon!(
///     brand_github[outline, filled],
///     user[outline]
/// );
/// // Generates:
/// // - Tabler::OUTLINE_BRAND_GITHUB
/// // - Tabler::FILLED_BRAND_GITHUB
/// // - Tabler::OUTLINE_USER
/// ```
///
/// With custom name:
/// ```ignore
/// use tabler_icon_definer::tabler_icon;
///
/// tabler_icon!(
///     #[name = "github"]
///     brand_github[outline, filled]
/// );
/// // Generates:
/// // - Tabler::OUTLINE_GITHUB
/// // - Tabler::FILLED_GITHUB
/// ```
///
/// With additional derives:
/// ```ignore
/// use tabler_icon_definer::tabler_icon;
///
/// tabler_icon!(
///     #[derive(serde::Serialize)]
///     #[name = "github"]
///     brand_github[outline, filled],
///     user[outline]
/// );
/// ```
/// Using generated enum:
/// ```ignore
/// let icon = Tabler::OUTLINE_GITHUB;
/// let svg = icon.as_str(); // Get SVG content
/// let name = icon.as_str_merget(); // Get icon name without style prefix
/// let full_name = icon.to_string(); // Get icon name with style prefix
/// ```
#[proc_macro]
pub fn tabler_icon(input: TokenStream) -> TokenStream {
    let TablerEnter {
        mut attrs,
        icons: mut icon_set,
    } = parse_macro_input!(input as TablerEnter);

    set_first_name_icon_by_name_attribut(&mut attrs, &mut icon_set);

    let default_derives = quote! {
          #[derive(Debug, Clone, PartialEq, Default)]
    };

    let attributes = if !attrs.is_empty() {
        quote! {
            #(#attrs)*
            #default_derives
        }
    } else {
        default_derives
    };

    let mut formated_ident = vec![]; // Вектор для форматированных идентификаторов
    let mut ident_from_str = vec![]; // Вектор для конвертации строки в идентификатор
    let mut ident_display_merget = vec![]; // Вектор для конвертации строки в идентификатор
    let mut ident_display = vec![]; // Вектор для представление идентификаторов
    let mut download_data = vec![]; // Вектор для данных загрузки

    for icon in icon_set.icons.iter() {
        let icon_name = icon.display_name.clone().unwrap_or(
            icon.name
                .to_string()
                .as_str()
                .trim_end_matches('_')
                .to_string(),
        );

        // Для каждого стиля иконки
        for style in icon.styles.iter() {
            // Добавляем пару (стиль, имя) в нижнем регистре
            download_data.push((
                style.to_string().to_lowercase(),
                icon.name
                    .to_string()
                    .trim_end_matches("_")
                    .to_lowercase()
                    .replace("_", "-"),
            ));

            let variant_name = format_ident!(
                "{}_{}",
                style.to_string().to_uppercase(),
                icon_name.to_uppercase()
            );

            let str_icon_name = format!(
                "{}_{}",
                style.to_string().to_lowercase(),
                icon_name.to_lowercase()
            );

            ident_from_str.push(quote! {
                #str_icon_name => Ok(Self::#variant_name),
            });

            ident_display.push(quote! {
                Self::#variant_name => write! (f, #str_icon_name)
            });

            ident_display_merget.push(quote! {
                Self::#variant_name => #icon_name
            });

            // Добавляем форматированный идентификатор в верхнем регистре
            formated_ident.push(format_ident!(
                "{}_{}",
                style.to_string().to_uppercase(),
                icon_name.to_uppercase()
            ));
        }
    }

    let as_str_match_arms: Vec<_> = formated_ident
        .iter()
        .enumerate()
        .map(|(num, formated_ident)| {
            let (style, name) = &download_data[num];
            let icon_url = download_link(style, name);

            let cache_filename = format!("{}_{}.svg", style, name)
                .replace("/", "_")
                .replace("\\", "_");
            let cache_path = get_cache_dir().join(cache_filename);
            let max_cache_age = Duration::from_secs(60 * 60 * 24 * 30); // 30 days

            let icon_content = if cache_path.exists() && is_cache_fresh(&cache_path, max_cache_age)
            {
                // use cached
                fs::read_to_string(&cache_path).expect("Failed to read cached icon")
            } else {
                // download and cache it
                let content = reqwest::blocking::get(&icon_url)
                    .expect("Failed to download icon")
                    .text()
                    .expect("Failed to read icon content");

                let content = content.trim_start();
                let content = {
                    if let Some(position) = content.find("<svg") {
                        &content[position..]
                    } else {
                        panic!(
                            "{} is not correct name with this style: {}!\nPlease check tabler for correct name!\nGot content:{:#?}",
                            name, style, &content
                        );
                    }
                }.to_string();

                fs::write(&cache_path, &content).expect("Failed to cache icon");
                content
            };

            quote! {
                Self::#formated_ident => {#icon_content}
            }
        })
        .collect();

    // Generate the final expanded code
    let expanded = quote! {
        /// Icon enum with embedded SVG content
        ///
        /// Each variant represents an icon with its specific style (outline/filled)
        /// The SVG content is embedded in the binary at compile time.
        #attributes
        pub enum Tabler {
            #[default]
            #(#formated_ident),*
        }

        impl Tabler {
            /// Returns the SVG content for the icon
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str_match_arms),*
                }
            }

            /// Return merget name
            ///
            /// undercust prefix befor `_`name
            pub fn as_str_merget(&self) -> &str {
                match self {
                    #(#ident_display_merget),*
                }
            }

            /// Returns a vector containing all available icon variants
            pub fn all_icons() -> Vec<Self> {
                vec![#(Self::#formated_ident),*]
            }

            #[cfg(feature = "leptos")]
            pub fn to_leptos(&self) -> leptos::prelude::AnyView {
                use leptos::html::div;
                use leptos::prelude::*;
                match self {
                    #(Self::#formated_ident => {
                        let svg = Self::#formated_ident.as_str();
                        leptos::prelude::view! {<div inner_html=svg />}.into_any()
                    })*
                }
            }
        }

        impl std::str::FromStr for Tabler {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let normalized = s.trim().to_lowercase();
                match normalized.as_str() {
                    #(#ident_from_str)*
                    _ => Err(format!("Unknown icon variant: {}", normalized))
                }
            }
        }

        impl std::fmt::Display for Tabler {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#ident_display),*
                }
            }
        }


    };

    expanded.into()
}

fn get_cache_dir() -> PathBuf {
    // Получаем различные переменные окружения Cargo
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());

    // Создаём структурированный путь кэша
    let cache_dir = PathBuf::from(target_dir).join("icon_cache");

    fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
    cache_dir
}

fn is_cache_fresh(
    cache_path: &PathBuf,
    max_age: Duration,
) -> bool {
    if let Ok(metadata) = fs::metadata(cache_path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = SystemTime::now().duration_since(modified) {
                return duration < max_age;
            }
        }
    }
    false
}

/// Construct download URL for icons
fn download_link(
    style_variant: &str,
    name: &str,
) -> String {
    format!(
        "https://raw.githubusercontent.com/tabler/tabler-icons/refs/heads/main/icons/{}/{}.svg",
        style_variant, name
    )
}

fn set_first_name_icon_by_name_attribut(
    attrs: &mut Vec<Attribute>,
    icon_set: &mut IconItem,
) {
    // Проверяем наличие иконок
    if icon_set.icons.is_empty() {
        panic!("Icon set is empty - must define at least one icon");
    }

    // Находим индекс атрибута name
    if let Some(name_index) = attrs
        .iter()
        .position(|attr| matches!(&attr.meta, Meta::NameValue(meta) if meta.path.is_ident("name")))
    {
        // Извлекаем атрибут по индексу
        let name_attr = attrs.remove(name_index);

        // Извлекаем значение строки из атрибута
        let name_str = match name_attr.meta {
            Meta::NameValue(name_value) => match name_value.value {
                Expr::Lit(lit) => match lit.lit {
                    Lit::Str(lit_str) => lit_str.value(),
                    _ => panic!("Invalid literal type in name attribute - expected string literal"),
                },
                _ => panic!("Expected literal expression in name attribute"),
            },
            _ => panic!("Unexpected meta format for name attribute"),
        };

        // Устанавливаем имя для первой иконки
        icon_set
            .icons
            .first_mut()
            .expect("Failed to get first icon")
            .display_name = Some(name_str);
    }
}
