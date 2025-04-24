use crate::{
    RW,
    components::{
        reusable_components::button::Button,
        toster::{ToastContext, ToastStore},
    },
    tools::gen_example_config,
};

use leptos::{
    IntoView,
    prelude::{Set, Update},
    view,
};
use leptos::{component, prelude::use_context};
use sss_core::{Data, types::provider::Tabler};

/// Кнопка для сброса настроек.
#[component]
pub fn RestoreButton() -> impl IntoView {
    let set_settings = use_context::<RW<Data>>().unwrap().1;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;

    view! {
        <Button
            alt=|| "Drop config".to_string()
            label=view! { {Tabler::OUTLINE_RESTORE.to_leptos()} }
            action=move || {
                set_settings.set(gen_example_config());
                store
                    .update(|x| {
                        x.push(ToastContext::Info("Configuration has restored!".to_string()))
                    });
            }
        />
    }
}
