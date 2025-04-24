use crate::{
    RW,
    components::{
        reusable_components::button::Button,
        toster::{ToastContext, ToastStore},
    },
};

use leptos::{IntoView, prelude::Update, view, wasm_bindgen::JsValue};
use leptos::{component, task::spawn_local};
use leptos::{prelude::use_context, web_sys};
use sss_core::{Data, types::provider::Tabler};
use sss_std::{prelude::HtmlLayouts, themes::Themes};
use wasm_bindgen_futures::JsFuture;

use crate::tools::SSSsetings;
/// Кнопка для загрузки настроек из буфера обмена.
#[component]
pub fn LoadButton() -> impl IntoView {
    let set_settings = use_context::<RW<Data>>().unwrap().1;
    let set_themes = use_context::<RW<Themes>>().unwrap().1;
    let set_layouts = use_context::<RW<HtmlLayouts>>().unwrap().1;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    view! {
        <Button
            alt=|| "Load config from bas64 by clipboard".to_string()
            label=view! { {Tabler::OUTLINE_CLIPBOARD.to_leptos()} }
            action=move || {
                {
                    spawn_local(async move {
                        let navigator = web_sys::window().expect("is it browser?").navigator();
                        let clipboard = navigator.clipboard();
                        match JsFuture::from(clipboard.read_text()).await {
                            Ok(value) => {
                                if let Some(value) = value.as_string() {
                                    if value.is_empty() {
                                        store
                                            .update(|x| {
                                                x
                                                    .push(
                                                        ToastContext::Warn("Your clipboard is empty".to_string()),
                                                    )
                                            });
                                        return;
                                    }
                                    let settings = SSSsetings::from_base64(&value);
                                    match settings {
                                        Ok(e) => {
                                            store
                                                .update(|x| {
                                                    x
                                                        .push(
                                                            ToastContext::Info("Your base64 has loaded!".to_string()),
                                                        )
                                                });
                                            e.update_context(set_settings, set_themes, set_layouts)
                                        }
                                        Err(e) => {
                                            store
                                                .update(|x| {
                                                    x
                                                        .push(
                                                            ToastContext::Error("Your base64 is't valid".to_string()),
                                                        )
                                                });
                                            web_sys::console::error_1(
                                                &JsValue::from_str(&e.to_string()),
                                            )
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                store
                                    .update(|x| {
                                        x
                                            .push(
                                                ToastContext::Error("Got error, check console".to_string()),
                                            )
                                    });
                                web_sys::console::error_1(&err);
                            }
                        }
                    });
                }
            }
        />
    }
}
