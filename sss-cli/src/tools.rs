use render::layout::Finalise;
use sss_core::{
    types::{
        link::Link,
        nickname::Nickname,
        provider::Provider,
        since::Since,
        skill::{Project, Skill},
        user::User,
    },
    Settings,
};
use sss_std::{
    converter::{pdf::html_to_pdf, png::html_to_image},
    prelude::Layouts,
    themes::Themes,
};
use tracing::{debug, info, instrument};

use crate::{settings::SSSCliSettings, HTML, PDF, PNG, SETTINGS};

pub type Result<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[instrument(skip(path, themes, layouts))]
#[inline]
/// Update setting
pub async fn refresh_settings(
    path: &str,
    themes: Option<&Themes>,
    layouts: Option<&Layouts>,
) -> Result {
    let mut settings = SSSCliSettings::load(path).await?;
    if let Some(themes) = themes {
        settings.themes = themes.to_owned()
    }
    if let Some(layouts) = layouts {
        settings.layouts = layouts.to_owned()
    }
    *SETTINGS.write().await = settings;
    Ok(())
}

#[instrument(skip(path, themes, layouts))]
#[inline]
/// Once load data
/// if cli has themes or || and layouts, replace it permanently
pub async fn refresh(
    path: &str,
    themes: Option<&Themes>,
    layouts: Option<&Layouts>,
) -> Result {
    debug!("Themes in cli: {:#?}", &themes);
    debug!("Layouts in cli: {:#?}", &layouts);
    refresh_settings(path, themes, layouts).await?;
    refresh_html().await?;
    refresh_png().await?;
    refresh_pdf().await?;
    Ok(())
}

#[instrument]
#[inline]
/// Generate final component
pub async fn refresh_html() -> Result {
    info!("Render HTML");
    let settings = SETTINGS.read().await;
    let layout = settings
        .layouts
        .to_layout(&settings.sss_user_settings, (&settings.themes).into());

    *HTML.write().await = layout.finalize()?;
    Ok(())
}

#[instrument]
#[inline]
/// Generate final component
pub async fn refresh_png() -> Result {
    info!("Render PNG");
    let html = HTML.read().await;
    let image = html_to_image(&html, None, 12).await?;
    *PNG.write().await = image;
    Ok(())
}

#[instrument]
#[inline]
/// Generate final component
pub async fn refresh_pdf() -> Result {
    info!("Render PDF");
    let html = HTML.read().await;
    let image = html_to_pdf(&html, None).await?;
    *PDF.write().await = image;
    Ok(())
}

#[inline]
pub fn gen_example_config() -> SSSCliSettings {
    SSSCliSettings {
        sss_user_settings: Settings {
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
                        provider: Provider::Github,
                        link: "https://github.com/your_nickname".to_string(),
                    },
                },
                Project {
                    name: "Cool Project".to_string(),
                    link: Link {
                        provider: Provider::Github,
                        link: "https://github.com/your_nickname".to_string(),
                    },
                },
            ],
            socials: vec![
                Link {
                    provider: Provider::Github,
                    link: "https://github.com/your_nickname".to_string(),
                },
                Link {
                    provider: Provider::Telegram,
                    link: "https://t.me/your_nickname".to_string(),
                },
            ],
            skills: vec![
                Skill {
                    skill: "Rust".to_string(),
                    projects: vec![Project {
                        name: "Cool Project".to_string(),
                        link: Link {
                            provider: Provider::Github,
                            link: "https://github.com/your_nickname".to_string(),
                        },
                    }],
                    since: Since {
                        start: 2020,
                        end: 0,
                    },
                    main: true,
                    repo_link: Link {
                        provider: Provider::Github,
                        link: "https://github.com/your_nickname".to_string(),
                    },
                },
                Skill {
                    skill: "JS/TS".to_string(),
                    projects: vec![Project {
                        name: "Cool Project".to_string(),
                        link: Link {
                            provider: Provider::Github,
                            link: "https://github.com/your_nickname".to_string(),
                        },
                    }],
                    since: Since {
                        start: 2020,
                        end: 2022,
                    },
                    main: false,
                    repo_link: Link {
                        provider: Provider::Github,
                        link: "https://github.com/your_nickname".to_string(),
                    },
                },
            ],
        },
        themes: Themes::default(),
        layouts: Layouts::default(),
    }
}
