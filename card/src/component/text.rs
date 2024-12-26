use std::borrow::Cow;

/// Just a text.
#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a> {
    pub text: Cow<'a, str>,
    pub font: Font,
}

impl<'a> Text<'a> {
    pub fn new(data: impl Into<Cow<'a, str>>, font: Option<Font>) -> Self {
        Self {
            text: data.into(),
            font: font.unwrap_or_default(),
        }
    }
}

/// Variants of font
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Font {
    Label,
    SubLabel,
    #[default]
    Text,
    Minor,
}
