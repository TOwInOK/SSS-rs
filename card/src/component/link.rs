use std::borrow::Cow;

use super::Component;

#[derive(Debug, Clone, PartialEq)]
pub struct Link<'a> {
    pub text: Option<Box<Component<'a>>>,
    pub href: Cow<'a, str>,
    pub icon: Option<Box<Component<'a>>>,
}

impl<'a> Link<'a> {
    pub fn new(
        text: Option<Component<'a>>,
        href: impl Into<Cow<'a, str>>,
        icon: Option<Component<'a>>,
    ) -> Self {
        Self {
            text: text.map(Box::new),
            href: href.into(),
            icon: icon.map(Box::new),
        }
    }
}
