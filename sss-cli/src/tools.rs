//! # Rendering and refresh utilities
//!
//! This module provides functions for rendering HTML, PNG, and PDF,
//! as well as refreshing configuration and managing global state.
//!
//! ## Main functions
//!
//! - `refresh_settings`: Load and update configuration from file
//! - `refresh`: Refresh all outputs based on service flags
//! - `refresh_html`: Generate HTML from current settings
//! - `refresh_png`: Generate PNG from current HTML
//! - `refresh_pdf`: Generate PDF from current HTML
//! - `gen_example_config`: Generate example configuration

use parser::parse::Loader;
use render::render::Render;
use sss_core::{
    Data, LayoutData,
    types::{
        link::Link,
        nickname::Nickname,
        provider::Tabler,
        since::Since,
        skill::{Project, Skill},
        user::User,
    },
};
use sss_std::{
    converter::{pdf::html_to_pdf, png::html_to_image},
    prelude::*,
};
use std::ops::Deref;
use tracing::{debug, error, info, instrument};

use crate::{
    HTML, PDF, PNG, SETTINGS,
    settings::{SSSCliSettings, services::Services},
};

pub type Result<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

/// Load configuration from file and update global settings
///
/// This function loads settings from the specified path, optionally overrides
/// theme/layout/services from CLI arguments, and updates the global [`SETTINGS`].
///
/// # Arguments
///
/// * `path` - Path to the configuration file (TOML or JSON)
/// * `themes` - Optional theme override from CLI
/// * `layouts` - Optional layout override from CLI
/// * `services` - Optional service flags override from CLI
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if loading fails
#[instrument(skip_all)]
#[inline]
pub async fn refresh_settings(
    path: &std::path::Path,
    themes: Option<&Themes>,
    layouts: Option<&HtmlLayouts>,
    services: Option<&Services>,
) -> Result<()> {
    info!("Settings is't actual!");

    let mut settings: SSSCliSettings = SSSCliSettings::load(path)?;
    if let Some(themes) = themes {
        settings.themes = themes.to_owned()
    }
    if let Some(layouts) = layouts {
        settings.layouts = layouts.to_owned()
    }
    if let Some(services) = services {
        settings.services = services.to_owned()
    }

    {
        *SETTINGS.deref().write().await = settings;
    }
    info!("Settings is actual now!");
    Ok(())
}
/// Status of actual data
pub enum SettingsUpdateType {
    // Is Actual
    AlreadyActual,
    // Needs to be updated
    NotActual,
}

/// Refresh all outputs based on service flags
///
/// This function loads settings and refreshes HTML/PNG/PDF based on which
/// services are enabled. HTML is always refreshed before any binary generation.
/// PNG and PDF are generated in background tasks.
///
/// # Arguments
///
/// * `path` - Path to the configuration file
/// * `themes` - Optional theme override from CLI
/// * `layouts` - Optional layout override from CLI
/// * `services` - Service flags to determine which outputs to refresh
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if loading/refreshing fails
#[instrument(skip_all)]
#[inline]
pub async fn refresh(
    path: &std::path::Path,
    themes: Option<&Themes>,
    layouts: Option<&HtmlLayouts>,
    services: Option<&Services>,
) -> Result {
    debug!("Themes in cli: {:#?}", &themes);
    debug!("Layouts in cli: {:#?}", &layouts);
    refresh_settings(path, themes, layouts, services).await?;

    let services = SETTINGS.read().await.services.clone();

    if services.html || services.png || services.pdf {
        refresh_html().await?;
    }
    if services.png {
        tokio::spawn(async move {
            if let Err(e) = refresh_png().await {
                error!("Got refresh error: {}", e);
            }
        });
    }
    if services.pdf {
        tokio::spawn(async move {
            if let Err(e) = refresh_pdf().await {
                error!("Got refresh error: {}", e);
            }
        });
    }
    Ok(())
}

#[instrument]
#[inline]
/// Generate final component
pub async fn refresh_html() -> Result {
    info!("Render HTML");
    {
        let settings = SETTINGS.read().await;
        let layout =
            HtmlTeraRender::new(&settings.data, (&settings.themes).into(), &settings.layouts)
                .finalize(&DefaultTemplates::STANDART);
        // let layout = settings
        //     .layouts
        //     .finalize(&settings.data, (&settings.themes).into())
        //     .layout()?;
        *HTML.write().await = layout.render()?.to_string();
    }
    info!("Done HTML");
    Ok(())
}

#[instrument]
#[inline]
/// Generate final component
pub async fn refresh_png() -> Result {
    info!("Render PNG");
    {
        let html = HTML.read().await.clone();
        let image = html_to_image(&html, None, 12).await?;
        *PNG.deref().write().await = image;
    }
    info!("Done PNG");
    Ok(())
}

#[instrument]
#[inline]
/// Generate final component
pub async fn refresh_pdf() -> Result {
    info!("Render PDF");
    {
        let html = HTML.read().await.clone();
        let image = html_to_pdf(&html, None).await?;
        *PDF.deref().write().await = image;
    }
    info!("Done PDF");
    Ok(())
}

pub fn gen_example_config() -> SSSCliSettings {
    SSSCliSettings {
        data: Data {
            layout: LayoutData {
                user: User {
                    name: "Your name".to_string(),
                    current_nickname: Nickname {
                        word: "Some Nickname".to_string(),
                        pronounce: "sÔme nIckname".to_string(),
                    },
                    prevision_nicknames: vec![Nickname {
                        word: "Your some old nickname".to_string(),
                        pronounce: "sÔme OLDER nIckname".to_string(),
                    }],
                },
                specifications: vec!["Your".to_string(), "cool".to_string(), "way".to_string()],
                about: "Something about yourself".to_string(),
                repos: vec![
                    Project {
                        name: "Cool Project".to_string(),
                        link: Link {
                            icon: Tabler::OUTLINE_GITHUB,
                            link: "https://github.com/your_nickname".to_string(),
                        },
                    },
                    Project {
                        name: "Cool Project".to_string(),
                        link: Link {
                            icon: Tabler::OUTLINE_GITHUB,
                            link: "https://github.com/your_nickname".to_string(),
                        },
                    },
                ],
                socials: vec![
                    Link {
                        icon: Tabler::OUTLINE_GITHUB,
                        link: "https://github.com/your_nickname".to_string(),
                    },
                    Link {
                        icon: Tabler::OUTLINE_TELEGRAM,
                        link: "https://t.me/your_nickname".to_string(),
                    },
                ],
                skills: vec![
                    Skill {
                        skill: "Rust".to_string(),
                        projects: vec![Project {
                            name: "Cool Project".to_string(),
                            link: Link {
                                icon: Tabler::OUTLINE_GITHUB,
                                link: "https://github.com/your_nickname".to_string(),
                            },
                        }],
                        since: Since {
                            start: 2020,
                            end: 0,
                        },
                        main: true,
                        repo_link: Link {
                            icon: Tabler::OUTLINE_GITHUB,
                            link: "https://github.com/your_nickname".to_string(),
                        },
                    },
                    Skill {
                        skill: "JS/TS".to_string(),
                        projects: vec![Project {
                            name: "Cool Project".to_string(),
                            link: Link {
                                icon: Tabler::OUTLINE_GITHUB,
                                link: "https://github.com/your_nickname".to_string(),
                            },
                        }],
                        since: Since {
                            start: 2020,
                            end: 2022,
                        },
                        main: false,
                        repo_link: Link {
                            icon: Tabler::OUTLINE_GITHUB,
                            link: "https://github.com/your_nickname".to_string(),
                        },
                    },
                ],
            },
        },
        ..Default::default()
    }
}
