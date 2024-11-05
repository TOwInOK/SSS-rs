/// just some div with text and Some([Component])
pub mod field;
/// Div with Some(border) contains [Component]
pub mod frame;
/// SVG icons
pub mod icon;
/// It's a button with link or an <a> el
pub mod link;
/// just a text
pub mod text;

use field::Field;
use frame::Frame;
use icon::Icon;
use link::Link;
use text::Text;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Component<'a> {
    Text(Text<'a>),
    Frame(Frame<'a>),
    Link(Link<'a>),
    Field(Field<'a>),
    Icon(Icon),
}

impl<'a> From<Text<'a>> for Component<'a> {
    fn from(value: Text<'a>) -> Self {
        Component::Text(value)
    }
}
impl<'a> From<Frame<'a>> for Component<'a> {
    fn from(value: Frame<'a>) -> Self {
        Self::Frame(value)
    }
}

impl<'a> From<Field<'a>> for Component<'a> {
    fn from(value: Field<'a>) -> Self {
        Self::Field(value)
    }
}
impl<'a> From<Link<'a>> for Component<'a> {
    fn from(value: Link<'a>) -> Self {
        Self::Link(value)
    }
}
impl<'a> From<Icon> for Component<'a> {
    fn from(value: Icon) -> Self {
        Self::Icon(value)
    }
}
