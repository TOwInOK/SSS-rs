/// just some div with text and Some([Component])
/// Div with Some(border) contains [Component]
pub mod frame;
/// SVG icons
pub mod icon;
/// It's a button with link or an <a> el
pub mod link;
/// just a text
pub mod text;

use frame::Frame;
use icon::Icon;
use link::Link;
use text::Text;
#[derive(Debug, Clone, PartialEq)]
pub enum Component<'a> {
    Text(Text<'a>),
    Frame(Frame<'a>),
    Link(Link<'a>),
    Icon(Icon),
}

impl<'a> From<Text<'a>> for Component<'a> {
    fn from(value: Text<'a>) -> Self {
        Self::Text(value)
    }
}
impl<'a> From<Frame<'a>> for Component<'a> {
    fn from(value: Frame<'a>) -> Self {
        Self::Frame(value)
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

#[macro_export]
macro_rules! text {
    ($data:expr, $font:ident) => {{
        use $crate::component::text::{Font, Text};
        Component::Text(Text::new($data, Some(Font::$font)))
    }};
    ($data:expr) => {{
        use $crate::component::text::Text;
        Component::Text(Text::new($data, None))
    }};
    ($($invalid:tt)*) => {
        compile_error!("Invalid text! macro usage. Use: text!(data) or text!(data, Font)")
    };
}
#[macro_export]
macro_rules! frame {
    ($aspect:ident; $component:expr) => {{
           use $crate::component::frame::{Frame, Direction};
           let components: Vec<Component> = $component;
           Frame::new(
               components,
               Direction::$aspect
           ).into()
       }};
    ($aspect:ident; $($component:expr),* $(,)?) => {{
           use $crate::component::frame::{Frame, Direction};
           let components: Vec<Component> = vec![$($component),*];
           Component::Frame(Frame::new(
               components,
               Direction::$aspect
           ))
       }};
    ($($component:expr),* $(,)?) => {{
        use $crate::component::frame::{Frame, Direction};
        Frame::new(
            vec![$($component),*],
            Direction::default()
        ).into()
    }};
    ($aspect:ident;) => {{
        use $crate::component::frame::{Frame, Direction};
        Frame::new(Vec::new(), Direction::$aspect).into()
    }};
    () => {{
        use $crate::component::frame::{Frame, Direction};
        Frame::new(Vec::new(), Direction::default()).into()
    }};
}
#[macro_export]
macro_rules! icon {
    // Для предопределенных иконок
    ($variant:ident, $aspect:ident) => {{
        use $crate::component::icon::{$variant, Icon};
        Icon::$variant($variant::$aspect).into()
    }};
    // Для кастомных иконок
    (custom, $svg:expr) => {{
        use $crate::component::icon::Icon;
        Icon::Custom($svg).into()
    }};
    // Ошибка
    ($($invalid:tt)*) => {
        compile_error!(
            "Invalid icon! macro usage. Use: icon!(Variant, Name) or icon!(custom, svg_string)"
        )
    };
}

#[macro_export]
macro_rules! field {
    // Только заголовок
    ($title:expr) => {{
        use $crate::component::field::Field;
        Field::new($title, None, None).into()
    }};
    // Заголовок и иконка
    ($title:expr, icon = $icon:expr) => {{
        use $crate::component::field::Field;
        Field::new($title, Some($icon), None).into()
    }};
    // Заголовок и элемент
    ($title:expr, element = $element:expr) => {{
        use $crate::component::field::Field;
        Field::new($title, None, Some($element)).into()
    }};
    // Полная версия
    ($title:expr, icon = $icon:expr, element = $element:expr) => {{
        use $crate::component::field::Field;
        Field::new($title, Some($icon), Some($element)).into()
    }};
    // Ошибка
    ($($invalid:tt)*) => {
        compile_error!("Invalid field! macro usage")
    };
}

#[macro_export]
macro_rules! link {
    // Только href
    ($href:expr) => {{
        use $crate::component::link::Link;
        Link::new(None, $href, None).into()
    }};
    // href и текст
    ($href:expr, text = $text:expr) => {{
        use $crate::component::link::Link;
        Link::new(Some($text), $href, None).into()
    }};
    // href и иконка
    ($href:expr, icon = $icon:expr) => {{
        use $crate::component::link::Link;
        Link::new(None, $href, Some($icon)).into()
    }};
    // Полная версия
    ($href:expr, text = $text:expr, icon = $icon:expr) => {{
        use $crate::component::link::Link;
        Link::new(Some($text), $href, Some($icon)).into()
    }};
    // Ошибка
    ($($invalid:tt)*) => {
        compile_error!("Invalid link! macro usage")
    };
}
