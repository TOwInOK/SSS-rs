#[cfg(test)]
mod gen_render_el {
    use std::sync::LazyLock;

    use card::{
        component::{frame::Frame, text::Text, Component},
        text,
    };
    use render::{
        layout::{self, finallyse},
        theme::umbrella::UMBRELLA_DARK,
    };
    use sss_core::{
        types::{
            render::Render,
            user::{About, Blank, Nickname, Skill, User},
        },
        Settings,
    };
    use tracing::{info, instrument};
    static TS: LazyLock<()> = LazyLock::new(|| {
        use tracing::Level;
        use tracing_subscriber::FmtSubscriber;

        tracing::subscriber::set_global_default(
            FmtSubscriber::builder()
                .with_max_level(Level::DEBUG)
                .pretty()
                .without_time()
                .finish(),
        )
        .expect("Fail to set global default subscriber");
    });

    #[test]
    #[instrument]
    fn text() {
        LazyLock::force(&TS);
        let components = vec![text!("some text")];
        info!("Components: {:#?}", &components);
        let component = Component::Frame(Frame {
            data: components,
            direction: card::component::frame::Direction::Vertical,
        });
        info!("Component: {:#?}", &component);
        let settings = Settings {
            user: User {
                name: "Some_Name".to_string(),
                nickname: Nickname {
                    word: "S".to_string(),
                    pronounce: Some("'asS".to_string()),
                },
                other_nicknames: vec![
                    Nickname {
                        word: "A".to_string(),
                        pronounce: Some("'aaI".to_string()),
                    },
                    Nickname {
                        word: "B".to_string(),
                        pronounce: Some("'bI".to_string()),
                    },
                ],
                specifications: vec!["xxx".to_string()],
                about: About {
                    text: "ssss".to_string(),
                    max_length: 20,
                },
                repos: vec![Blank {
                    provider: "GH".to_string(),
                    link: "0".to_string(),
                    logo: None,
                    main: false,
                }],
                social_media: vec![Blank {
                    provider: "GgH".to_string(),
                    link: "02".to_string(),
                    logo: None,
                    main: false,
                }],
                skills: vec![Skill {
                    skill: "02".to_string(),
                    top_projects: vec![Blank {
                        provider: "GH".to_string(),
                        link: "0".to_string(),
                        logo: None,
                        main: false,
                    }],
                    since: None,
                    main: false,
                    maintainer_site: None,
                }],
            },
            render_type: Render::HTML,
        };
        let r = finallyse(&settings, &UMBRELLA_DARK);
        match r {
            layout::RenderOut::HTML(e) => std::fs::write("example.html", &e).unwrap(),
            layout::RenderOut::LEPTOS(_) => todo!(),
            layout::RenderOut::HTMLCSS(e) => std::fs::write("example.html", &e).unwrap(),
        }
    }
}
