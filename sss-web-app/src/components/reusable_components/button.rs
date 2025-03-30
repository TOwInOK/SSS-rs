use leptos::prelude::*;
use sss_std::prelude::*;

use crate::RW;

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
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let css = "border font-bold".to_string();
    view! {
        <button
            title=alt
            on:click=move |_| {
                action()
            }
            class=css
            style=move || format!(
                "background-color: {}; color: {}",
                themes.get().colors().text,
                themes.get().colors().background
            )
        >{label}</button>
    }
}

/// Add button
#[component]
pub fn AddButton<A: Fn() + 'static, Alt: Fn() -> String + 'static + Send>(
    action: A,
    alt: Alt,
) -> impl IntoView {
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let css = "border font-bold p-4".to_string();
    view! {
        <button
            title=alt
            on:click=move |_| {
                action()
            }
            class=css
            style=move || format!(
                "background-color: {}; color: {}",
                themes.get().colors().text,
                themes.get().colors().background
            )
        >+</button>
    }
}

/// Remove button
#[component]
pub fn RemoveButton<A: Fn() + 'static, Alt: Fn() -> String + 'static + Send>(
    action: A,
    alt: Alt,
) -> impl IntoView {
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let css = "border font-bold".to_string();
    view! {
        <button
            title=alt
            on:click=move |_| {
                action()
            }
            class=css
            style=move || format!(
                "background-color: {}; color: {}",
                themes.get().colors().text,
                themes.get().colors().background
            )
        >x</button>
    }
}
