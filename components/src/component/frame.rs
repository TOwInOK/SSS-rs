use super::Component;
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub data: Vec<Component>,
    pub direction: Direction,
}

impl Frame {
    pub fn new(data: Vec<Component>, direction: Direction) -> Frame {
        Self { data, direction }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    /// Vertical placement
    ///
    /// from top to bottom
    #[default]
    Vertical,
    /// Horizontal placement from
    ///
    /// left to right
    Horizontal,
    /// Reversed vertical placement
    ///
    /// from bottom to top
    ReversVertical,
    /// Reversed Horizontal placement
    ///
    /// from right to left
    ReversHorizontal,
}
