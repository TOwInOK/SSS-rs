use crate::{
    RW,
    components::{
        reusable_components::button::Button,
        toster::{ToastContext, ToastStore},
    },
};

use leptos::component;
use leptos::prelude::use_context;
use leptos::{
    IntoView,
    prelude::{Set, Update},
    view,
};
use sss_core::{Data, types::provider::Tabler};

/// Кнопка для сброса настроек.
#[component]
pub fn DropButton() -> impl IntoView {
    let set_settings = use_context::<RW<Data>>().unwrap().1;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    view! {
        <Button
            alt=|| "Drop config".to_string()
            label=view! { {Tabler::OUTLINE_TRASH.to_leptos()} }
            action=move || {
                set_settings.set(Data::default());
                store
                    .update(|x| {
                        x.push(ToastContext::Info("Configuration has dropped!".to_string()))
                    });
            }
        />
    }
}
