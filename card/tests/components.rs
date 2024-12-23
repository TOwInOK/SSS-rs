#[cfg(test)]
mod components {
    use card::{
        component::{
            field::Field,
            frame::{Direction, Frame},
            icon::{Filled, Icon, Outline},
            link::Link,
            text::{Font, Text},
            Component,
        },
        create_icon, icon, text,
    };

    #[test]
    fn text() {
        let text = Text::new("some text", Some(Font::default()));
        assert_eq!(text.text, "some text");
        assert_eq!(text.font, Font::Text);

        let text = text!("some text");
        assert_eq!(
            text,
            Component::Text(Text::new("some text", Some(Font::Text)))
        );
    }
    #[test]
    fn link() {
        let text = Component::Text(Text::new("some text", Some(Font::Label)));
        let icon = Component::Icon(Icon::Filled(Filled::GitHub));
        let link = Link::new(Some(text.clone()), "some ref", Some(icon.clone()));
        assert_eq!(link.href, "some ref");
        assert_eq!(link.text.unwrap(), Box::new(text));
        assert_eq!(link.icon.unwrap(), Box::new(icon));
    }
    #[test]
    fn frame() {
        let frame = Frame::new(vec![], Direction::Vertical);
        assert!(frame.data.is_empty());
        assert_eq!(frame.direction, Direction::Vertical);
    }
    #[test]
    fn field() {
        let text = Component::Text(Text::new("some text", Some(Font::Label)));
        let tt = text!("some text");
        let field = Field::new(&tt, None, Some(&icon!(Filled, GitHub)));
        assert!(field.element.is_none());
        assert_eq!(field.title, &text);
    }
    #[test]
    fn test_icon() {
        let icon = create_icon!(Outline, GitHub);
        assert_eq!(icon, Icon::Outline(Outline::GitHub));
    }
}
