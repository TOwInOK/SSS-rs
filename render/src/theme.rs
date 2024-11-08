pub mod umbrella;

type Color = &'static str;
/// Night, Light
pub struct Theme {
    // Main colors
    pub primary: Color,
    pub secondary: Color,
    pub thirdly: Color,
    // Border
    pub border: Color,
    // Padding
    pub button_pudding: usize,
    pub border_pudding: usize,
    pub frame_pudding: usize,
    // Gap
    pub button_gap: usize,
    pub border_gap: usize,
    pub frame_gap: usize,
}

pub enum Shade {
    Dark(Theme),
    Light(Theme),
}
