use leptos::prelude::*;
/// Ограничитель для прокручиваемых элементов по горизонтали.
#[component]
pub fn ScrollXBar(children: Children) -> impl IntoView {
    view! {
        <div class="grid">
            <div class="grid grid-flow-col gap-2 overflow-x-scroll snap-x will-change-scroll">
                {children()}
            </div>
        </div>
    }
}
