use parser::parse::parse;
use render::{layout::Finalize, theme::Theme};
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
use sss_std::{prelude::Layouts, themes::Themes};
use tracing::instrument;

use crate::{HTML, SETTINGS, THEME};

#[instrument]
pub async fn refresh_settings(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let settings = parse(path)?;
    let mut new_settings = SETTINGS.write().await;
    *new_settings = settings;
    Ok(())
}

#[instrument]
pub async fn refresh_theme(themes: &Themes) {
    let theme: &'static Theme = themes.into();
    *THEME.write().await = theme;
}

#[instrument]
pub async fn load(
    path: &str,
    themes: &Themes,
    layouts: &Layouts,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    refresh_settings(path).await?;
    refresh_theme(themes).await;
    refresh_html(layouts).await?;
    Ok(())
}

pub fn gen_example_config() -> Settings {
    Settings {
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
    }
}

#[instrument]
pub async fn layout_build(
    layouts: &Layouts
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let theme = THEME.read().await;
    let settings = SETTINGS.read().await;
    let layout = layouts.to_layout(&settings, &theme);
    layout.finalize()
}

#[instrument]
pub async fn refresh_html(
    layouts: &Layouts
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let html = layout_build(layouts).await?;
    *HTML.write().await = html;
    Ok(())
}
