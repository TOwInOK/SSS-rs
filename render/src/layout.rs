use crate::theme::Shading;

pub trait Layout<Out, C, S>
where
    S: Shading,
{
    /// Transforms the profile layout into the requested output format
    fn render(
        sections: C,
        shade: &S,
    ) -> Out;

    fn finylize(
        rendered: Out,
        shade: &S,
    ) -> Out;
}
