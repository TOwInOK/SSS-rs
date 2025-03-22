#[inline]
/// Generate css by html via config
pub fn gen_css(
    config: &encre_css::Config,
    document: &str,
) -> String {
    encre_css::generate([document], config)
}
