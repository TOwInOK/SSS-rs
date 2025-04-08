use std::borrow::Cow;

use sss_core::Data;

use crate::layout::{Layout, Limitations};

/// Abstract trait for rendering objects into a any type.
pub trait Render<T = String>
where
    T: Clone + ToString,
    Self: Layout + Limitations + FilterLimitations,
{
    /// Render the object into a `Cow<T>`.
    fn render(&self) -> super::Result<Cow<T>>;
}

pub trait FilterLimitations
where
    Self: Limitations,
{
    fn filter(&self) -> Cow<Data>;
}
