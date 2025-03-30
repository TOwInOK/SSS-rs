use leptos::web_sys::{Event, HtmlTextAreaElement};
use leptos::{ev::Targeted, prelude::*};
/// Поле для ввода многострочного текста, с заданным действием `action` при изменении и начальным значением `prop`.
#[component]
pub fn TextArea<
    I: Fn(Targeted<Event, HtmlTextAreaElement>) + 'static,
    P: Fn() -> String + 'static + Send,
    L: Fn() -> usize + 'static + Send + Sync,
    Alt: Fn() -> String + 'static + Send,
    Placeholder: Fn() -> String + 'static + Send,
>(
    action: I,
    prop: P,
    maxlength: L,
    alt: Alt,
    placeholder: Placeholder,
) -> impl IntoView {
    view! {
        <textarea
            class="border indent-2"
            on:input:target=move |ev| { action(ev) }
            prop:value=prop
            title=alt
            placeholder=placeholder
            maxlength=maxlength
        />
    }
}
