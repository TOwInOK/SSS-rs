pub mod umbrella;

type Color = &'static str;
type TailWindStyle = &'static str;
/// Night, Light
pub struct Theme {
    // Text's styles
    label_style: TailWindStyle,
    sub_label_style: TailWindStyle,
    text_style: TailWindStyle,
    text_minor_style: TailWindStyle,
    // Link's style
    link_style: TailWindStyle,
    // Field's style
    frame_style: TailWindStyle,
    // Icon's style
    icon_style: TailWindStyle,
    // Frame's style
    vertical_frame_style: TailWindStyle,
    horizontal_frame_style: TailWindStyle,
    revers_vertical_frame_style: TailWindStyle,
    revers_horizontal_frame_style: TailWindStyle,
    // Main colors
    primary: Color,
    secondary: Color,
    thirdly: Color,
    // Border
    /// hex
    border: Color,
    // Padding
    button_pudding: usize,
    border_pudding: usize,
    frame_pudding: usize,
    // Gap
    button_gap: usize,
    border_gap: usize,
    frame_gap: usize,
}
// TODO: Make theme generator
impl Theme {
    pub fn label(&self) -> String {
        todo!()
    }
    pub fn sub_label(&self) -> String {
        todo!()
    }
    pub fn text(&self) -> String {
        todo!()
    }
    pub fn text_minor(&self) -> String {
        todo!()
    }
    pub fn horizontal_frame(&self) -> String {
        todo!()
    }
    pub fn reversed_horizontal_frame(&self) -> String {
        todo!()
    }
    pub fn vertical_frame(&self) -> String {
        todo!()
    }
    pub fn reversed_vertical_frame(&self) -> String {
        todo!()
    }
    pub fn link(&self) -> String {
        todo!()
    }
    pub fn field(&self) -> String {
        todo!()
    }
    pub fn icon(&self) -> String {
        todo!()
    }
}

pub enum Shade {
    Dark(Theme),
    Light(Theme),
}
