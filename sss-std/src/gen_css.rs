pub fn gen_css(
    config: Option<encre_css::Config>,
    document: &str,
) -> String {
    let config = config.unwrap_or_default();
    encre_css::generate([document], &config)
}
