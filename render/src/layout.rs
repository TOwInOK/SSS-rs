use sss_core::UserProvider;

use crate::theme::Shading;

pub trait Layout<Out, C, S, D>
where
    D: UserProvider + UserProvider,
    S: Shading,
{
    /// Transforms the profile layout into the requested output format
    fn render(component: C, shade: &S, data: &D) -> Out;
}
