use super::Component;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Link<'a> {
    pub text: Option<&'a Component<'a>>,
    pub href: &'a str,
    pub icon: Option<&'a Component<'a>>,
}

impl<'a> Link<'a> {
    pub fn new(
        text: Option<&'a Component<'a>>,
        href: &'a str,
        icon: Option<&'a Component<'a>>,
    ) -> Self {
        Self { text, href, icon }
    }
}
