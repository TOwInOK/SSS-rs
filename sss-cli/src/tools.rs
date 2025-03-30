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

#[instrument(skip_all)]
#[inline]
/// Update setting
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

#[instrument(skip_all)]
#[inline]
/// Once load data
/// if cli has themes or || and layouts, replace it permanently
pub async fn refresh(
    path: &std::path::Path,
    themes: Option<&Themes>,
    layouts: Option<&HtmlLayouts>,
    services: Option<&Services>,
) -> Result {
    debug!("Themes in cli: {:#?}", &themes);
    debug!("Layouts in cli: {:#?}", &layouts);
    refresh_settings(path, themes, layouts, services).await?;

    let services = &SETTINGS.read().await.services;

    if services.html {
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
        let html = HTML.read().await;
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
        let html = HTML.read().await;
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
