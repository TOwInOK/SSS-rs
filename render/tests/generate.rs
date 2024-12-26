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
    fn gen() {
        LazyLock::force(&TS);
        let components = vec![text!("test")];
        info!("Components: {:#?}", &components);
        let component = Component::Frame(Frame {
            data: components,
            direction: card::component::frame::Direction::Vertical,
        });
        info!("Component: {:#?}", &component);
        let settings = Settings {
            user: User {
                name: "Test_User".to_string(),
                nickname: Nickname {
                    word: "T".to_string(),
                    pronounce: Some("test".to_string()),
                },
                other_nicknames: vec![
                    Nickname {
                        word: "Test1".to_string(),
                        pronounce: Some("test1".to_string()),
                    },
                    Nickname {
                        word: "Test2".to_string(),
                        pronounce: Some("test2".to_string()),
                    },
                ],
                specifications: vec!["test_spec".to_string()],
                about: About {
                    text: "test description".to_string(),
                    max_length: 20,
                },
                repos: vec![Blank {
                    provider: "Test".to_string(),
                    link: "test_link".to_string(),
                    logo: None,
                    main: false,
                }],
                social_media: vec![Blank {
                    provider: "TestSocial".to_string(),
                    link: "test_social_link".to_string(),
                    logo: None,
                    main: false,
                }],
                skills: vec![Skill {
                    skill: "test_skill".to_string(),
                    top_projects: vec![Blank {
                        provider: "Test".to_string(),
                        link: "test_project_link".to_string(),
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
