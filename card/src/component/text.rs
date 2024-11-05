/// Just a text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text<'a> {
    pub text: &'a str,
    pub font: Font,
}

impl<'a> Text<'a> {
    pub fn new(data: &'a str, font: Font) -> Self {
        Self { text: data, font }
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
