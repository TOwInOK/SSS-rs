#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Icon {
    #[default]
    GitHub,
}

impl Icon {
    /// Convert variant to svg
    pub fn as_str(&self) -> &str {
        match &self {
            Icon::GitHub => todo!(),
        }
    }
}
