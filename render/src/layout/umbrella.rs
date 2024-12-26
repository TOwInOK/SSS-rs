use super::Layout;
use crate::component_layout::ComponentLayout;
use card::prelude::*;
use sss_core::UserProvider;
pub struct Umbrella;

impl Layout for Umbrella {
    fn layout<'a>(
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a> {
        frame!(Vertical;
            Self::user(component_layout, settings),
            Self::specializations(component_layout, settings),
            Self::about(component_layout, settings),
            Self::skills(component_layout, settings),
            Self::socials(component_layout, settings)
        )
    }

    fn user<'a>(
        _component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a> {
        let user = settings.user();
        frame!(Horizontal;
            text!(&user.name, Label),
            text!(" (", Minor),
            text!(&user.nickname.word),
            text!(")", Minor)
        )
    }

    fn specializations<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        frame!(Vertical;
            text!("Specializations:", Label),
            frame!(Horizontal;
                icon!(Filled, GitHub),
                text!("Backend Developer")
            )
        )
    }

    fn about<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        field!(text!("About me"), element = text!("I love coding in Rust!"))
    }

    fn skills<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        frame!(Vertical;
            text!("Skills:", Label),
            frame!(Horizontal;
                text!("Rust"),
                text!("WebAssembly"),
                text!("TypeScript")
            )
        )
    }

    fn socials<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        frame!(Vertical;
            text!("Find me on:", Label),
            frame!(Horizontal;
                link!("https://github.com",
                    text = text!("GitHub"),
                    icon = icon!(Filled, GitHub)
                ),
                link!("https://example.com",
                    text = text!("Website")
                )
            )
        )
    }

    fn specialization<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        frame!(Horizontal;
            icon!(Filled, GitHub),
            text!("Specialization")
        )
    }

    fn skill<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        frame!(Horizontal;
            text!("Skill"),
            text!("(5/5)", Minor)
        )
    }

    fn social<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        frame!(Horizontal;
            icon!(Filled, GitHub),
            text!("GitHub")
        )
    }
}
