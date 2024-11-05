#[cfg(test)]
mod components {
    use card::component::{
        field::Field,
        frame::{Direction, Frame},
        link::Link,
        svg::{Icons, Svg},
        text::{Font, Text},
    };

    #[test]
    fn text() {
        let text = Text::new("some text", Font::default());
        assert_eq!(text.text, "some text");
        assert_eq!(text.font, Font::Text);
    }
    #[test]
    fn link() {
        let link = Link::new(
            Some(Text::new("some text", Font::Label)),
            "some ref",
            Some(Svg::new(Icons::GitHub)),
        );
        assert_eq!(link.href, "some ref");
        assert_eq!(link.text.unwrap().text, "some text");
        assert_eq!(link.text.unwrap().font, Font::Label);
        assert_eq!(link.icon.unwrap().data, Icons::GitHub);
    }
    #[test]
    fn frame() {
        let frame = Frame::new(&[], Direction::VStack);
        assert!(frame.data.is_empty());
        assert_eq!(frame.direction, Direction::VStack);
    }
    #[test]
    fn field() {
        let field = Field::new(
            Text::new("some text", Font::Text),
            None,
            Some(Svg::new(Icons::GitHub)),
        );
        assert!(field.element.is_none());
        assert_eq!(field.title.text, "some text");
        assert_eq!(field.title.font, Font::Text);
        assert_eq!(field.icon.unwrap().data, Icons::GitHub);
    }
    #[test]
    fn svg() {
        let svg = Svg::new(Icons::GitHub);
        assert_eq!(svg.data, Icons::GitHub);
    }
}
