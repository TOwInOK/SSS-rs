use leptos::prelude::*;
#[component]
/// Limiter for scrollable box/items
pub fn ScrollXBar(children: Children) -> impl IntoView {
    view! {
        <div class="flex w-full">
            <div class="flex gap-2 overflow-x-scroll snap-x">
                {children()}
            </div>
        </div>
    }
}
