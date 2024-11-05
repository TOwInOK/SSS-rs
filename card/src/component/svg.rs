#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Svg {
    pub data: Icons,
}

impl Svg {
    pub fn new(data: Icons) -> Self {
        Self { data }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Icons {
    #[default]
    GitHub,
}

impl Icons {
    /// Convert variant to svg
    pub fn as_str(&self) -> &str {
        match &self {
            Icons::GitHub => todo!(),
        }
    }
}
