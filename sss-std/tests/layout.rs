use std::fs;

use render::prelude::*;
use sss_core::{
    types::{
        link::Link, nickname::Nickname, provider::Provider, since::Since, skill::Skill, user::User,
    },
    Settings,
};
use sss_std::prelude::*;

#[test]
fn test_umbrella_layout() {
    let settings = Settings {
        user: User {
            name: "Дмитрий".to_string(),
            current_nickname: Nickname {
                word: "TOwInOK".to_string(),
                pronounce: "ту́винок".to_string(),
            },
            ..Default::default()
        },
        specifications: vec![
            "Backend Development".to_string(),
            "Full-Stack Development".to_string(),
            "Systems Programming".to_string(),
        ],
        about: "Учу находить пиво в холодильнике".to_string(),
        repos: vec![Link {
            provider: Provider::Github,
            link: "https://github.com/TOwInOK".to_string(),
        }],
        socials: vec![
            Link {
                provider: Provider::Github,
                link: "https://github.com/TOwInOK".to_string(),
            },
            Link {
                provider: Provider::Telegram,
                link: "https://t.me/TOwInOK".to_string(),
            },
        ],
        skills: vec![
            Skill {
                skill: "Rust".to_string(),
                projects: vec![Link {
                    provider: Provider::Github,
                    link: "https://github.com/TOwInOK/sss-rs".to_string(),
                }],
                since: Since {
                    start: 2018,
                    end: 0, // означает "по настоящее время"
                },
                main: true,
                repo_link: Link {
                    provider: Provider::Github,
                    link: "https://crates.io/users/TOwInOK".to_string(),
                },
            },
            Skill {
                skill: "JS/TS".to_string(),
                projects: vec![],
                since: Since {
                    start: 2019,
                    end: 0,
                },
                main: false,
                repo_link: Link {
                    provider: Provider::Github,
                    link: "#".to_string(),
                },
            },
        ],
    };

    let ub = UmbrellaHtmlTeraRender {
        settings: &settings,
        theme: &UMBRELLA,
    };
    let html = ub.finalize().unwrap();

    fs::write("card2.html", html).unwrap();
}
