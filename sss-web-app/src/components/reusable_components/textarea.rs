use leptos::web_sys::{Event, HtmlTextAreaElement};
use leptos::{ev::Targeted, prelude::*};
/// Поле для ввода многострочного текста, с заданным действием `action` при изменении и начальным значением `prop`.
#[component]
pub fn TextArea<
    I: Fn(Targeted<Event, HtmlTextAreaElement>) + 'static,
    P: Fn() -> String + 'static + Send,
    L: Fn() -> usize + 'static + Send + Sync + Copy,
    CL: Fn() -> usize + 'static + Send + Sync + Copy,
    Alt: Fn() -> String + 'static + Send,
    Placeholder: Fn() -> String + 'static + Send,
>(
    action: I,
    prop: P,
    maxlength: L,
    current_length: CL,
    alt: Alt,
    placeholder: Placeholder,
) -> impl IntoView {
    view! {
        <div class="w-full flex flex-col">
            <p class=move || {
                if current_length() as f32 / maxlength() as f32 > 0.7 {
                    "opacity-80  text-[var(--color-accent)]"
                } else {
                    "opacity-80"
                }
            }>{move || format!("{}/{}", current_length(), maxlength())}</p>
            <textarea
                on:input:target=move |ev| { action(ev) }
                prop:value=prop
                title=alt
                placeholder=placeholder
                maxlength=maxlength
            />
        </div>
    }
}
