use leptos::web_sys::{Event, HtmlInputElement};
use leptos::{ev::Targeted, prelude::*};
#[component]
pub fn Input<
    I: Fn(Targeted<Event, HtmlInputElement>) + 'static,
    P: Fn() -> String + 'static + Send,
>(
    action: I,
    prop: P,
) -> impl IntoView {
    view! {
        <input
            class="border"
            on:input:target=action
                prop:value=prop
        />
    }
}
