pub mod html;
pub mod html_css;
pub mod leptos;
use crate::theme::Shading;
use card::component::Component;

pub trait Renderer<T: Shading> {
    type Output;
    fn render(theme: &T, component: &Component) -> Self::Output;
    fn finallyse(theme: &T, component: Self::Output) -> Self::Output;
}
