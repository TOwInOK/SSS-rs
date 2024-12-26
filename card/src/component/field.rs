use super::Component;
#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a> {
    pub title: Box<Component<'a>>,
    pub icon: Option<Box<Component<'a>>>,
    pub element: Option<Box<Component<'a>>>,
}

impl<'a> Field<'a> {
    pub fn new(
        title: Component<'a>,
        icon: Option<Component<'a>>,
        element: Option<Component<'a>>,
    ) -> Self {
        Self {
            title: Box::new(title),
            icon: icon.map(Box::new),
            element: element.map(Box::new),
        }
    }
}
