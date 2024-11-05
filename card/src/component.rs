/// just some div with text and Some([Component])
pub mod field;
/// Div with Some(border) contains [Component]
pub mod frame;
/// It's a button with link or an <a> el
pub mod link;
/// SVG icons
pub mod svg;
/// just a text
pub mod text;

use field::Field;
use frame::Frame;
use link::Link;
use svg::Svg;
use text::Text;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Component<'a> {
    Text(Text<'a>),
    Frame(Frame<'a>),
    Link(Link<'a>),
    Field(Field<'a>),
    Icon(Svg),
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
impl<'a> From<Svg> for Component<'a> {
    fn from(value: Svg) -> Self {
        Self::Icon(value)
    }
}
