use leptos::prelude::*;
#[component]
/// Box that will be scrolled
pub fn ScrollableBox(children: Children) -> impl IntoView {
    view! {
        <div class="grid gap-2 snap-start">
        {
            children()
        }
        </div>
    }
}
