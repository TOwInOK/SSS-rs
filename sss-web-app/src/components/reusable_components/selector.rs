use leptos::prelude::*;
use sss_std::{prelude::HtmlLayouts, themes::Themes};

use crate::RW;

/// Селектор для выбора темы оформления.
#[component]
pub fn ThemeSelector() -> impl IntoView {
    let (get, set) = use_context::<RW<Themes>>().unwrap();

    let current = get.get_untracked();
    let items = std::iter::once(current.clone())
        .chain(
            Themes::all_themes()
                .iter()
                .filter(|theme| *theme != &current)
                .cloned(),
        )
        .collect::<Vec<_>>();

    view! {
        <div class="grid grid-flow-col gap-2 items-center border p-1.5 justify-center">
            <div>Themes</div>
            <select
            class="truncate max-w-48 items-center w-full colored"
                on:change=move |ev| {
                    if let Ok(value) = event_target_value(&ev).parse() {
                        set.update(|x| *x = value);
                    }
                }
                prop:value=move || get.read().to_string()
            >
            {
                items.into_iter().map(|category| {
                    view! {
                        <option value=category.to_string()>
                            {category.to_string()}
                        </option>
                    }
                }).collect::<Vec<_>>()
            }
            </select>
        </div>
    }
}

/// Селектор для выбора макета карточки.
#[component]
pub fn LayoutSelector() -> impl IntoView {
    let (get, set) = use_context::<RW<HtmlLayouts>>().unwrap();
    let current = get.get_untracked();
    let items = std::iter::once(current.clone())
        .chain(
            HtmlLayouts::all()
                .iter()
                .filter(|theme| *theme != &current)
                .cloned(),
        )
        .collect::<Vec<_>>();
    view! {
        <div class=" grid grid-flow-col gap-2 items-center self-center appearance-auto border p-1.5 justify-center">
            <div>Layouts</div>
            <select
                class="truncate max-w-48 items-center w-full colored"
                on:change=move |ev| {
                    if let Ok(value) = event_target_value(&ev).parse() {
                        set.update(|x| *x = value);
                    }
                }
                prop:value=move || get.read().to_string()
            >
            {
                items.into_iter().map(|category| {
                    view! {
                        <option value=category.to_string()>
                            {category.to_string()}
                        </option>
                    }
                }).collect::<Vec<_>>()
            }
            </select>
        </div>
    }
}

/// Селектор для выбора иконки.
#[component]
pub fn IconSelector<A, P>(
    action: A,
    prop: P,
) -> impl IntoView
where
    A: Fn(leptos::web_sys::Event) + 'static,
    P: Fn() -> String + 'static + Send,
{
    let providers = sss_core::types::provider::Tabler::all_icons();
    view! {
        <div class=" grid grid-flow-col gap-2 items-center self-center appearance-auto border p-1.5 justify-between">
            <div>Icon :</div>
            <select
            class="border"
                on:change=action
                prop:value=prop
            >
            {
                providers.iter().map(|category| {
                    view! {
                        <option value=category.to_string()
                        class="colored"
                        >
                            {category.to_string()}
                        </option>
                    }
                }).collect_view()
            }
            </select>
        </div>
    }
}
