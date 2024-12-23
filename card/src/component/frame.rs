use super::Component;
#[derive(Debug, Clone, PartialEq)]
pub struct Frame<'a> {
    pub data: Vec<Component<'a>>,
    pub direction: Direction,
}

impl<'a> Frame<'a> {
    pub fn new(data: Vec<Component<'a>>, direction: Direction) -> Self {
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
