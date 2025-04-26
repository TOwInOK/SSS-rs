use leptos::web_sys::{Event, HtmlInputElement};
use leptos::{ev::Targeted, prelude::*};
#[component]
pub fn Input<
    I: Fn(Targeted<Event, HtmlInputElement>) + 'static + Send + Sync + Copy,
    P: Fn() -> String + 'static + Send + Sync + Copy,
    ML: Fn() -> usize + 'static + Send + Sync + Copy,
    CL: Fn() -> usize + 'static + Send + Sync + Copy,
    Alt: Fn() -> String + 'static + Send + Sync + Copy,
    Placeholder: Fn() -> String + 'static + Send + Sync + Copy,
>(
    action: I,
    prop: P,
    alt: Alt,
    placeholder: Placeholder,
    current_length: CL,
    maxlength: ML,
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
            <input
                on:input:target=action
                prop:value=prop
                title=alt
                placeholder=placeholder
                maxlength=maxlength
            />
        </div>
    }
}

#[component]
pub fn InputNumeric<
    I: Fn(Targeted<Event, HtmlInputElement>) + 'static + Copy + Send + Sync,
    P: Fn() -> String + 'static + Send + Sync + Copy,
    Alt: Fn() -> String + 'static + Send + Sync + Copy,
    Placeholder: Fn() -> String + 'static + Send + Sync + Copy,
>(
    action: I,
    prop: P,
    alt: Alt,
    placeholder: Placeholder,
) -> impl IntoView {
    view! {
        <input
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
    I: Fn(Targeted<Event, HtmlInputElement>) + 'static + Send + Sync + Copy,
    P: Fn() -> String + 'static + Send + Sync + Copy,
    Alt: Fn() -> String + 'static + Send + Sync + Copy,
    Placeholder: Fn() -> String + 'static + Send + Sync + Copy,
>(
    action: I,
    prop: P,
    alt: Alt,
    placeholder: Placeholder,
) -> impl IntoView {
    view! {
        <input
            on:input:target=action
            prop:value=prop
            title=alt
            placeholder=placeholder
            type="url"
        />
    }
}
