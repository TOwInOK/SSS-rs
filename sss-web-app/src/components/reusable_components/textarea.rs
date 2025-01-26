use leptos::web_sys::{Event, HtmlTextAreaElement};
use leptos::{ev::Targeted, prelude::*};
#[component]
pub fn TextArea<
    I: Fn(Targeted<Event, HtmlTextAreaElement>) + 'static,
    P: Fn() -> String + 'static + Send,
>(
    action: I,
    prop: P,
) -> impl IntoView {
    view! {
        <textarea
            class="border"
            on:input:target=move |ev| { action(ev) }
                prop:value=prop
        />
    }
}
