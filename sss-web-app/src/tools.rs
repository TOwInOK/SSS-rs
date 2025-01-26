use base64_light::{base64_decode_str, base64_encode};
use leptos::prelude::{Get, ReadSignal, Set, WriteSignal};
use serde::{Deserialize, Serialize};
use sss_core::Settings;
use sss_std::{prelude::Layouts, themes::Themes};

#[inline]
pub fn gen_example_config() -> Settings {
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
    Settings {
        user: User {
            name: "Dmitry".to_string(),
            current_nickname: Nickname {
                word: "TOwInOK".to_string(),
                pronounce: "тУвинок".to_string(),
            },
            prevision_nicknames: vec![Nickname {
                word: "nqcq".to_string(),
                pronounce: "нкьюСиКью".to_string(),
            }],
        },
        specifications: vec![
            "Full-Stack developer".to_string(),
            "Q/A".to_string(),
            "DevOps".to_string(),
        ],
        about: "Software developer without soul, only mind...".to_string(),
        repos: vec![
            Project {
                name: "SSS-rs".to_string(),
                link: Link {
                    provider: Provider::Github,
                    link: "https://github.com/TOwInOK/SSS-rs".to_string(),
                },
            },
            Project {
                name: "zen-rs".to_string(),
                link: Link {
                    provider: Provider::Github,
                    link: "https://github.com/TOwInOK/zen-rs".to_string(),
                },
            },
        ],
        socials: vec![
            Link {
                provider: Provider::Github,
                link: "https://github.com/TOwInOK".to_string(),
            },
            Link {
                provider: Provider::Telegram,
                link: "https://t.me/towinok".to_string(),
            },
        ],
        skills: vec![
            Skill {
                skill: "Rust".to_string(),
                projects: vec![Project {
                    name: "SSS-rs".to_string(),
                    link: Link {
                        provider: Provider::Github,
                        link: "https://github.com/TOwInOK/SSS-rs".to_string(),
                    },
                }],
                since: Since {
                    start: 2020,
                    end: 0,
                },
                main: true,
                repo_link: Link {
                    provider: Provider::Github,
                    link: "https://github.com/TOwInOK".to_string(),
                },
            },
            Skill {
                skill: "JS/TS".to_string(),
                projects: vec![Project {
                    name: "GSM-example".to_string(),
                    link: Link {
                        provider: Provider::Github,
                        link: "https://github.com/TOwInOK/Game-Sample-Market".to_string(),
                    },
                }],
                since: Since {
                    start: 2022,
                    end: 2025,
                },
                main: false,
                repo_link: Link {
                    provider: Provider::Github,
                    link: "https://github.com/TOwInOK".to_string(),
                },
            },
        ],
    }
}

/// As part of cli system
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename = "settings")]
pub struct SSSsetings {
    /// User specific settings
    #[serde(rename = "user")]
    #[serde(default)]
    pub s: Settings,

    /// Theme configuration
    #[serde(rename = "theme")]
    #[serde(default)]
    pub t: Themes,

    /// Layout configuration
    #[serde(rename = "layout")]
    #[serde(default)]
    pub l: Layouts,
}

impl SSSsetings {
    pub fn new(s: Settings, t: Themes, l: Layouts) -> Self {
        Self { s, t, l }
    }
    pub fn to_base64(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(base64_encode(&self.to_toml()?))
    }
    pub fn from_base64(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_toml(&base64_decode_str(value))
    }
    fn to_toml(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(toml::to_string(self)?)
    }
    fn from_toml(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(toml::from_str(value)?)
    }
    pub fn update_context(&self, s: W<Settings>, t: W<Themes>, l: W<Layouts>) {
        s.set(self.s.clone());
        t.set(self.t.clone());
        l.set(self.l.clone());
    }

    pub fn from_context(s: R<Settings>, t: R<Themes>, l: R<Layouts>) -> Self {
        Self {
            s: s.get(),
            t: t.get(),
            l: l.get(),
        }
    }
}

type W<T> = WriteSignal<T>;
type R<T> = ReadSignal<T>;
