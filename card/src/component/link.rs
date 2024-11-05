use super::{icon::Icon, text::Text};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Link<'a> {
    pub text: Option<Text<'a>>,
    pub href: &'a str,
    pub icon: Option<Icon>,
}

impl<'a> Link<'a> {
    pub fn new(text: Option<Text<'a>>, href: &'a str, icon: Option<Icon>) -> Self {
        Self { text, href, icon }
    }
}
