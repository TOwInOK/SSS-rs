use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextStyle {
    Ghost,
    #[default]
    Default,
    Primary,
}

#[component]
/// Section with items
pub fn Text(
    title: &'static str,
    #[prop(default = TextStyle::Default)] style: TextStyle,
    #[prop(default = true)] indented: bool,
    #[prop(default = false)] inverted: bool,
) -> impl IntoView {
    let style = match style {
        TextStyle::Ghost => "opacity-80",
        TextStyle::Default => "",
        TextStyle::Primary => "font-bold",
    };
    let colored = { if inverted { "colored" } else { "colored-inverted" } };
    view! {
            <p class=format!("pl-2 {} {} {}", if indented { "pl-2" } else { "" }, style, colored)
            >
                {title}
            </p>
    }
}
