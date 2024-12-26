use super::Layout;
use crate::component_layout::ComponentLayout;
use card::prelude::*;
use sss_core::UserProvider;
pub struct Umbrella;

impl Layout for Umbrella {
    fn layout<'a>(
        &self,
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a> {
        frame!(Vertical;
            self.user(component_layout, settings),
            self.specializations(component_layout, settings),
            self.about(component_layout, settings),
            self.skills(component_layout, settings),
            self.socials(component_layout, settings)
        )
    }

    fn user<'a>(
        &self,
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
        &self,
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
        &self,
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        field!(text!("About me"), element = text!("I love coding in Rust!"))
    }

    fn skills<'a>(
        &self,
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
        &self,
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

    // Вспомогательные методы можно оставить как todo!()
    fn specialization<'a>(
        &self,
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        todo!()
    }

    fn skill<'a>(
        &self,
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        todo!()
    }

    fn social<'a>(
        &self,
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        todo!()
    }
}
