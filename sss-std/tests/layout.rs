use std::{fs, path::Path};

use render::layout::Layout;
use sss_core::{
    Settings,
    types::{
        link::Link,
        nickname::Nickname,
        provider::Tabler,
        since::Since,
        skill::{Project, Skill},
        user::User,
    },
};
#[cfg(feature = "image_generation")]
use sss_std::converter::{pdf::html_to_pdf, png::html_to_image};
use sss_std::prelude::*;

#[test]
fn test_umbrella_layout() {
    let settings = def_set();
    let ub = Layouts::UMBRELLA.finalize(&settings, &CATPPUCCIN_MAUVE);
    let html = ub.render().unwrap();
    fs::write(Path::new("./card.html"), html).unwrap();
}
#[cfg(feature = "image_generation")]
#[tokio::test]
async fn create_image() {
    let settings = def_set();
    let ub = Layouts::CASTLE.finalize(&settings, &CATPPUCCIN_MAUVE);
    let html = ub.render().unwrap();
    let img = html_to_image(&html, None, 12).await.unwrap();
    fs::write(Path::new("./img.png"), img).unwrap();
}
#[cfg(feature = "image_generation")]
#[tokio::test]
async fn create_pdf() {
    let settings = def_set();
    let ub = Layouts::CASTLE.finalize(&settings, &CATPPUCCIN_MAUVE);
    let html = ub.render().unwrap();
    let pdf = html_to_pdf(&html, None).await.unwrap();
    fs::write(Path::new("./pdf.pdf"), pdf).unwrap();
}

fn def_set() -> Settings {
    Settings {
        user: User {
            name: "Дмитрий".to_string(),
            current_nickname: Nickname {
                word: "TOwInOK".to_string(),
                pronounce: "ту́винок".to_string(),
            },
            prevision_nicknames: vec![Nickname {
                word: "TOwInOK".to_string(),
                pronounce: "ту́винок".to_string(),
            }],
        },
        specifications: vec![
            "Backend Development".to_string(),
            "Full-Stack Development".to_string(),
            "Systems Programming".to_string(),
        ],
        about: "Учу находить пиво в холодильнике".to_string(),
        repos: vec![
            Project {
                name: "Some".to_string(),
                link: Link {
                    icon: Tabler::OUTLINE_GITHUB,
                    link: "https://github.com/your_nickname".to_string(),
                },
            },
            Project {
                name: "Cool".to_string(),
                link: Link {
                    icon: Tabler::OUTLINE_GITHUB,
                    link: "https://github.com/your_nickname".to_string(),
                },
            },
            Project {
                name: "Project".to_string(),
                link: Link {
                    icon: Tabler::OUTLINE_GITHUB,
                    link: "https://github.com/your_nickname".to_string(),
                },
            },
        ],
        socials: vec![
            Link {
                icon: Tabler::OUTLINE_GITHUB,
                link: "https://github.com/TOwInOK".to_string(),
            },
            Link {
                icon: Tabler::OUTLINE_TELEGRAM,
                link: "https://t.me/TOwInOK".to_string(),
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
                    start: 2018,
                    end: 0, // означает "по настоящее время"
                },
                main: true,
                repo_link: Link {
                    icon: Tabler::OUTLINE_GITHUB,
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
                    icon: Tabler::OUTLINE_GITHUB,
                    link: "#".to_string(),
                },
            },
        ],
    }
}
