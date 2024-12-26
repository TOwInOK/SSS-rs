#[cfg(test)]
mod gen_render_el {
    use std::sync::LazyLock;

    use card::{
        component::{frame::Frame, icon::Icon, Component},
        text,
    };
    use render::{
        component_layout::html::HtmlRenderer,
        layout::{self, umbrella::Umbrella, Layout},
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
        let components = vec![text!("test")]
            .into_iter()
            .map(std::borrow::Cow::Owned)
            .collect::<Vec<_>>();
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
                    logo: Some(
                        Icon::Filled(card::component::icon::Filled::GitHub)
                            .as_str()
                            .to_string(),
                    ),
                    main: true,
                }],
                social_media: vec![Blank {
                    provider: "TestSocial".to_string(),
                    link: "test_social_link".to_string(),
                    logo: Some(
                        Icon::Filled(card::component::icon::Filled::GitHub)
                            .as_str()
                            .to_string(),
                    ),
                    main: true,
                }],
                skills: vec![Skill {
                    skill: "test_skill".to_string(),
                    top_projects: vec![Blank {
                        provider: "Test".to_string(),
                        link: "test_project_link".to_string(),
                        logo: Some(
                            Icon::Filled(card::component::icon::Filled::GitHub)
                                .as_str()
                                .to_string(),
                        ),
                        main: true,
                    }],
                    since: None,
                    main: true,
                    maintainer_site: None,
                }],
            },
            render_type: Render::HTML,
        };
        let r = Umbrella::render(&settings, &HtmlRenderer, &settings, &UMBRELLA_DARK);
        let r = Umbrella::render_finaly(r, &UMBRELLA_DARK);
        match r {
            layout::RenderOut::HTML(e) => std::fs::write("example.html", &e).unwrap(),
            layout::RenderOut::LEPTOS(_) => todo!(),
            layout::RenderOut::HTMLCSS(e) => std::fs::write("example.html", &e).unwrap(),
        }
    }
}
