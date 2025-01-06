use crate::theme::Shade;

pub trait Layout<Out, C, S>
where
    S: Shade,
{
    /// Transforms the profile layout into the requested output format
    fn render(
        &self,
        sections: C,
        shade: &S,
    ) -> Out;

    // fn finylize(
    //     rendered: Out,
    //     shade: &S,
    // ) -> Out;
}
