use std::{ops::Deref, str::FromStr};

use super::Layout;
use crate::component_layout::ComponentLayout;
use card::prelude::*;
use sss_core::{types::user::Blank, UserProvider};
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
            Self::socials(component_layout, settings),
            Self::skills(component_layout, settings)
        )
    }

    fn user<'a>(
        _component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a> {
        let user = settings.user();
        frame!(Horizontal;
            text!(&user.name, Label),
            frame!(Horizontal;
                text!("[", Label),
                frame!(Vertical;
                    text!(&user.nickname.word),
                    text!(&user.nickname.pronounce.unwrap_or_default(), SubLabel)
                ),
                text!("]", Label)
            )
        )
    }

    fn specializations<'a>(
        _component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a> {
        let user = settings.user();
        frame!(Horizontal;
            frame!(Vertical;
                user.specifications.iter().map(|spec| {
                    frame!(Horizontal;
                        text!("[", Label),
                        text!(spec),
                        text!("]", Label)
                    )
                }).collect()
            ),
            text!(&user.about.text)
        )
    }

    fn socials<'a>(
        component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a> {
        let user = settings.user();
        let items = {
            let mut items: Vec<Component> = vec![];
            for item in user.social_media.iter() {
                let icon = Component::Icon(
                    Icon::from_str(item.logo.clone().unwrap_or_default().as_str()).unwrap(),
                );
                let text = item.provider.to_lowercase();
                items.push( frame!(Horizontal; text!("[", Label), frame!(Vertical; icon, text!(text)), text!("]", Label)))
                ;
            }
            items
        };

        frame!(Horizontal;items)
    }

    fn skills<'a>(
        _component_layout: &'a impl ComponentLayout,
        settings: &'a impl UserProvider,
    ) -> Component<'a> {
        let user = settings.user();

        frame!(Horizontal;
            user.skills.iter().map(|skill| {
                frame!(Vertical;
                    text!(&skill.skill, Label),
                    text!(format!("{} -> today", &skill.since.unwrap_or_default()), SubLabel),
                    text!(&skill.maintainer_site.clone().unwrap_or_default(), Minor)
                )
            }).collect()
        )
    }

    // Эти методы можно удалить, так как они не используются в новом дизайне
    fn about<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        Component::Frame(Frame {
            data: vec![],
            direction: Direction::Vertical,
        })
    }

    fn specialization<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        Component::Frame(Frame {
            data: vec![],
            direction: Direction::Horizontal,
        })
    }

    fn skill<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        Component::Frame(Frame {
            data: vec![],
            direction: Direction::Horizontal,
        })
    }

    fn social<'a>(
        _component_layout: &'a impl ComponentLayout,
        _settings: &'a impl UserProvider,
    ) -> Component<'a> {
        Component::Frame(Frame {
            data: vec![],
            direction: Direction::Horizontal,
        })
    }
}
