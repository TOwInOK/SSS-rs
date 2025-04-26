use leptos::prelude::*;

use crate::components::reusable_components::prelude::ScrollXBar;
#[component]
/// Inner section in [Section]
pub fn Stack(
    title: impl ToString,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="grid gap-2 p-4 border">
            <p class="pl-2 font-bold">{title.to_string()}</p>
            {children()}
        </div>
    }
}

#[component]
/// Inner section in [Section]
pub fn DynamicStack<
    ML: Fn() -> usize + 'static + Send + Sync + Copy,
    CL: Fn() -> usize + 'static + Send + Sync + Copy,
>(
    title: impl ToString,
    current_length: CL,
    max_length: ML,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="grid gap-2 p-4 border">
            <div class="flex gap-2">
                <p class="pl-2 font-bold">{title.to_string()}</p>
                <p>|</p>
                <p class=move || {
                    if current_length() as f32 / max_length() as f32 > 0.7 {
                        "opacity-80  text-[var(--color-accent)]"
                    } else {
                        "opacity-80"
                    }
                }>{move || format!("{}/{}", current_length(), max_length())}</p>
            </div>
            <ScrollXBar>{children()}</ScrollXBar>

        </div>
    }
}
