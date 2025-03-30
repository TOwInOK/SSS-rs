use crate::RW;
use leptos::prelude::*;
use sss_std::themes::Themes;

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
    let themes = use_context::<RW<Themes>>().unwrap().0;

    let style = match style {
        TextStyle::Ghost => "opacity-80",
        TextStyle::Default => "",
        TextStyle::Primary => "font-bold",
    };
    let css = move || {
        if inverted {
            format!(
                "background-color: {}; color: {}",
                themes.get().colors().text,
                themes.get().colors().background
            )
        } else {
            format!(
                "background-color: {}; color: {}",
                themes.get().colors().background,
                themes.get().colors().text
            )
        }
    };
    view! {
            <p class=format!("pl-2 {} {}", if indented { "pl-2" } else { "" }, style)
                style=css
            >
                {title}
            </p>
    }
}
