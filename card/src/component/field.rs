use super::Component;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field<'a> {
    pub title: &'a Component<'a>,
    pub icon: Option<&'a Component<'a>>,
    pub element: Option<&'a Component<'a>>,
}

impl<'a> Field<'a> {
    pub fn new(
        title: &'a Component<'a>,
        icon: Option<&'a Component<'a>>,
        element: Option<&'a Component<'a>>,
    ) -> Self {
        Self {
            title,
            icon,
            element,
        }
    }
}
