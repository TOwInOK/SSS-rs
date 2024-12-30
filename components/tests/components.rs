mod components {
    use components::{
        component::{
            frame::{Direction, Frame},
            icon::{Filled, Icon, Outline},
            link::Link,
            text::{Font, Text},
            Component,
        },
        create_icon, text,
    };

    #[test]
    fn text() {
        let text = Text::new("some text", Some(Font::default()));
        assert_eq!(text.text, "some text");
        assert_eq!(text.font, Font::Text);

        let text: Component = text!("some text");
        assert_eq!(
            text,
            Component::Text(Text::new("some text", Some(Font::Text)))
        );
    }
    #[test]
    fn link() {
        let text = Component::Text(Text::new("some text", Some(Font::Label)));
        let icon = Component::Icon(Icon::Filled(Filled::GitHub));
        let link = Link::new(
            Some(text.clone()),
            "some ref".to_string(),
            Some(icon.clone()),
        );
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
    fn test_icon() {
        let icon = create_icon!(Outline, GitHub);
        assert_eq!(icon, Icon::Outline(Outline::GitHub));
    }
    #[test]
    fn rec_frame() {
        let text = Component::Text(Text::new("some text", Some(Font::Label)));
        let frame = Frame::new(
            vec![
                Component::Text(Text::new("some text", Some(Font::Label))),
                Component::Text(Text::new("some text", Some(Font::Label))),
                text,
            ],
            Direction::Vertical,
        );
        assert!(frame.data.is_empty());
        assert_eq!(frame.direction, Direction::Vertical);
    }
}
