/// Allow to build a component to string
pub trait Layout {
    /// Render layout
    fn render(&self) -> crate::Result<String>;
}
