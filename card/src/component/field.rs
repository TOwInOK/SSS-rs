use super::{icon::Icon, text::Text, Component};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field<'a> {
    pub title: Text<'a>,
    pub icon: Option<Icon<'a>>,
    pub element: Option<&'a Component<'a>>,
}

impl<'a> Field<'a> {
    pub fn new(
        title: Text<'a>,
        element: Option<&'a Component<'a>>,
        icon: Option<Icon<'a>>,
    ) -> Self {
        Self {
            title,
            element,
            icon,
        }
    }
}
