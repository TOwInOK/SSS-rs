//! Gen layout of [User]

use card::{
    component::{
        text::{Font, Text},
        Component,
    },
    text,
};
use sss_core::Settings;

pub fn render(settings: &Settings) -> Component<'_> {
    let user = &settings.user;
    // Username label
    let username_label = Component::Text(Text::new(&user.name, Some(Font::Label)));
    let username_label = text!(&user.name, Label);
    let username_label = text!(&user.name);
    username_label
}
