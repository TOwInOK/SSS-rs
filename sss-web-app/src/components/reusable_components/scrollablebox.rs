use leptos::prelude::*;
/// Контейнер с возможностью прокрутки, элементы которого выравниваются по сетке.
#[component]
pub fn ScrollableBox(children: Children) -> impl IntoView {
    view! { <div class="grid gap-2 snap-center">{children()}</div> }
}
