use base64_light::{base64_decode_str, base64_encode};
use leptos::prelude::{Get, Set, Signal, WriteSignal};
use serde::{Deserialize, Serialize};
use sss_core::Data;
use sss_std::prelude::{HtmlLayouts, Themes};

#[inline]
pub fn gen_example_config() -> Data {
    use sss_core::{
        LayoutData,
        types::{
            link::Link,
            nickname::Nickname,
            provider::Tabler,
            since::Since,
            skill::{Project, Skill},
            user::User,
        },
    };
    Data {
        layout: LayoutData {
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
                        icon: Tabler::OUTLINE_GITHUB,
                        link: "https://github.com/TOwInOK/SSS-rs".to_string(),
                    },
                },
                Project {
                    name: "zen-rs".to_string(),
                    link: Link {
                        icon: Tabler::OUTLINE_GITHUB,
                        link: "https://github.com/TOwInOK/zen-rs".to_string(),
                    },
                },
            ],
            socials: vec![
                Link {
                    icon: Tabler::OUTLINE_GITHUB,
                    link: "https://github.com/TOwInOK".to_string(),
                },
                Link {
                    icon: Tabler::OUTLINE_DISCORD,
                    link: "https://discordapp.com/users/965639480797184021".to_string(),
                },
                Link {
                    icon: Tabler::OUTLINE_TELEGRAM,
                    link: "https://t.me/towinok".to_string(),
                },
            ],
            skills: vec![
                Skill {
                    skill: "Rust".to_string(),
                    projects: vec![Project {
                        name: "SSS-rs".to_string(),
                        link: Link {
                            icon: Tabler::OUTLINE_GITHUB,
                            link: "https://github.com/TOwInOK/SSS-rs".to_string(),
                        },
                    }],
                    since: Since {
                        start: 2020,
                        end: 0,
                    },
                    main: true,
                    repo_link: Link {
                        icon: Tabler::OUTLINE_GITHUB,
                        link: "https://github.com/TOwInOK".to_string(),
                    },
                },
                Skill {
                    skill: "JS/TS".to_string(),
                    projects: vec![Project {
                        name: "GSM-example".to_string(),
                        link: Link {
                            icon: Tabler::OUTLINE_GITHUB,
                            link: "https://github.com/TOwInOK/Game-Sample-Market".to_string(),
                        },
                    }],
                    since: Since {
                        start: 2022,
                        end: 2025,
                    },
                    main: false,
                    repo_link: Link {
                        icon: Tabler::OUTLINE_GITHUB,
                        link: "https://github.com/TOwInOK".to_string(),
                    },
                },
            ],
        },
    }
}

/// As part of cli system
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename = "settings")]
pub struct SSSsetings {
    /// User specific settings
    #[serde(rename = "user")]
    #[serde(default)]
    pub s: Data,

    /// Theme configuration
    #[serde(rename = "theme")]
    #[serde(default)]
    pub t: Themes,

    /// Layout configuration
    #[serde(rename = "layout")]
    #[serde(default)]
    pub l: HtmlLayouts,
}

impl SSSsetings {
    pub fn new(
        s: Data,
        t: Themes,
        l: HtmlLayouts,
    ) -> Self {
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
    pub fn update_context(
        &self,
        s: W<Data>,
        t: W<Themes>,
        l: W<HtmlLayouts>,
    ) {
        s.set(self.s.clone());
        t.set(self.t.clone());
        l.set(self.l.clone());
    }

    pub fn from_context(
        s: R<Data>,
        t: R<Themes>,
        l: R<HtmlLayouts>,
    ) -> Self {
        Self {
            s: s.get(),
            t: t.get(),
            l: l.get(),
        }
    }
}

type W<T> = WriteSignal<T>;
type R<T> = Signal<T>;
