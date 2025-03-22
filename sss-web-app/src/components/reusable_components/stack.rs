use leptos::prelude::*;
#[component]
/// Inner section in [Section]
pub fn Stack(
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="grid gap-2 p-4 border">
            <p class="pl-2 font-bold">{title}</p>
            {children()}
        </div>
    }
}
