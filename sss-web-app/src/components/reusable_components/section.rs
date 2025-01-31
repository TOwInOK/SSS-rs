use crate::RW;
use leptos::prelude::*;
use sss_std::themes::Themes;
#[component]
/// Section with items
pub fn Section(title: &'static str, children: Children) -> impl IntoView {
    let themes = use_context::<RW<Themes>>().unwrap().0;
    view! {
        <div class="grid gap-4 p-1.5 border  overflow-clip z-20"
            style=move || format!(
                "background-color: {};",
                themes.get().colors().secondary
            )
        >
            <p class="pl-2 font-bold"
                style=move || format!(
                    "background-color: {}; color: {}",
                    themes.get().colors().primary,
                    themes.get().colors().secondary
                )
            >
                {title}
            </p>
            {children()}
        </div>
    }
}
