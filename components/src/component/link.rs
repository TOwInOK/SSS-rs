use super::Component;

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub text: Option<Box<Component>>,
    pub href: String,
    pub icon: Option<Box<Component>>,
}

impl Link {
    pub fn new(text: Option<Component>, href: String, icon: Option<Component>) -> Self {
        Self {
            text: text.map(Box::new),
            href,
            icon: icon.map(Box::new),
        }
    }
}
