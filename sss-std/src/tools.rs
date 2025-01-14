pub fn gen_css(
    config: &encre_css::Config,
    document: &str,
) -> String {
    encre_css::generate([document], config)
}

/// inject css via style for html content
pub fn css_inject(
    content: String,
    css: String,
) -> String {
    format!("{content}\n<style>\n{css}\n</style>")
}
