/// Just a text.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub font: Font,
}

impl Text {
    pub fn new(data: impl Into<String>, font: Option<Font>) -> Self {
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
