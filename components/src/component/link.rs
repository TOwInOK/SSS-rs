use super::Component;

#[derive(Debug, Clone, PartialEq)]
pub struct Link<'a> {
    pub text: Option<Box<Component<'a>>>,
    pub href: &'a str,
    pub icon: Option<Box<Component<'a>>>,
}

impl<'a> Link<'a> {
    pub fn new(text: Option<Component<'a>>, href: &'a str, icon: Option<Component<'a>>) -> Self {
        Self {
            text: text.map(Box::new),
            href,
            icon: icon.map(Box::new),
        }
    }
}
