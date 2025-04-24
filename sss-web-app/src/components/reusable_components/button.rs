use leptos::prelude::*;

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Default,
    Add,
    Remove,
}

/// Blank absorber
#[component]
pub fn Button<A: Fn() + 'static, Alt: Fn() -> String + 'static + Send>(
    #[prop(optional)] label: Option<AnyView>,
    action: A,
    alt: Alt,
    #[prop(default = ButtonStyle::Default)] style: ButtonStyle,
) -> impl IntoView {
    match style {
        ButtonStyle::Default => DefaultButton(DefaultButtonProps {
            label,
            action,
            alt,
        })
        .into_any(),
        ButtonStyle::Add => AddButton(AddButtonProps {
            action,
            alt,
        })
        .into_any(),
        ButtonStyle::Remove => RemoveButton(RemoveButtonProps {
            action,
            alt,
        })
        .into_any(),
    }
}

#[component]
pub fn DefaultButton<A: Fn() + 'static, Alt: Fn() -> String + 'static + Send>(
    #[prop(optional)] label: Option<impl IntoView>,
    action: A,
    alt: Alt,
) -> impl IntoView {
    view! {
        <button title=alt on:click=move |_| { action() } class="button">
            {label}
        </button>
    }
}

#[component]
pub fn AddButton<A: Fn() + 'static, Alt: Fn() -> String + 'static + Send>(
    action: A,
    alt: Alt,
) -> impl IntoView {
    view! {
        <button title=alt on:click=move |_| { action() } class="button button-add">
            +
        </button>
    }
}

#[component]
pub fn RemoveButton<A: Fn() + 'static, Alt: Fn() -> String + 'static + Send>(
    action: A,
    alt: Alt,
) -> impl IntoView {
    view! {
        <button title=alt on:click=move |_| { action() } class="button">
            x
        </button>
    }
}
