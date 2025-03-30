use std::borrow::Cow;

use crate::layout::{Layout, Limitations};

/// Abstract trait for rendering objects into a any type.
pub trait Render<T = String>
where
    T: Clone + ToString,
    Self: Layout + Limitations,
{
    /// Render the object into a `Cow<T>`.
    fn render(&self) -> super::Result<Cow<T>>;
}
