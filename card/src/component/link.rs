use super::{svg::Svg, text::Text};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Link<'a> {
    pub text: Option<Text<'a>>,
    pub href: &'a str,
    pub icon: Option<Svg>,
}

impl<'a> Link<'a> {
    pub fn new(text: Option<Text<'a>>, href: &'a str, icon: Option<Svg>) -> Self {
        Self { text, href, icon }
    }
}
