use super::{svg::Svg, text::Text, Component};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field<'a> {
    pub title: Text<'a>,
    pub icon: Option<Svg>,
    pub element: Option<&'a Component<'a>>,
}

impl<'a> Field<'a> {
    pub fn new(title: Text<'a>, element: Option<&'a Component<'a>>, icon: Option<Svg>) -> Self {
        Self {
            title,
            element,
            icon,
        }
    }
}
