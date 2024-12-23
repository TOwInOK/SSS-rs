pub mod html;
pub mod html_css;
pub mod leptos;
use crate::theme::{CssShading, TailwindShading};
use card::component::Component;

pub trait Renderer<T: TailwindShading + CssShading> {
    type Output;
    fn render(theme: &T, component: &Component) -> Self::Output;
    fn finallyse(theme: &T, component: Self::Output) -> Self::Output;
}
