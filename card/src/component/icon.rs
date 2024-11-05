#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Icon {
    GitHub(IconVariant),
}

impl Icon {
    /// Convert variant to svg
    pub fn as_str(&self) -> &str {
        match &self {
            Icon::GitHub(_) => todo!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum IconVariant {
    #[default]
    Outline,
    Filled,
}
