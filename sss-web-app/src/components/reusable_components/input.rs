use leptos::web_sys::{Event, HtmlInputElement};
use leptos::{ev::Targeted, prelude::*};
#[component]
pub fn Input<
    I: Fn(Targeted<Event, HtmlInputElement>) + 'static,
    P: Fn() -> String + 'static + Send + Sync,
    L: Fn() -> usize + 'static + Send + Sync,
    Alt: Fn() -> String + 'static + Send + Sync,
    Placeholder: Fn() -> String + 'static + Send + Sync,
>(
    action: I,
    prop: P,
    alt: Alt,
    placeholder: Placeholder,
    maxlength: L,
) -> impl IntoView {
    view! {
        <input
            class="border indent-2"
            on:input:target=action
            prop:value=prop
            title=alt
            placeholder=placeholder
            maxlength=maxlength
        />
    }
}

#[component]
pub fn InputNumeric<
    I: Fn(Targeted<Event, HtmlInputElement>) + 'static,
    P: Fn() -> String + 'static + Send + Sync,
    Alt: Fn() -> String + 'static + Send + Sync,
    Placeholder: Fn() -> String + 'static + Send + Sync,
>(
    action: I,
    prop: P,
    alt: Alt,
    placeholder: Placeholder,
) -> impl IntoView {
    view! {
        <input
            class="border indent-2"
            on:input:target=action
            prop:value=prop
            title=alt
            placeholder=placeholder
            type="number"
        />
    }
}

#[component]
pub fn InputUrl<
    I: Fn(Targeted<Event, HtmlInputElement>) + 'static,
    P: Fn() -> String + 'static + Send + Sync,
    Alt: Fn() -> String + 'static + Send + Sync,
    Placeholder: Fn() -> String + 'static + Send + Sync,
>(
    action: I,
    prop: P,
    alt: Alt,
    placeholder: Placeholder,
) -> impl IntoView {
    view! {
        <input
            class="border indent-2"
            on:input:target=action
            prop:value=prop
            title=alt
            placeholder=placeholder
            type="url"
        />
    }
}
