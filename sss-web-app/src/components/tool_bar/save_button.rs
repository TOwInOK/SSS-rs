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
/// Кнопка для сохранения настроек в буфер обмена.
#[component]
pub fn SaveButton() -> impl IntoView {
    let settings = use_context::<RW<Data>>().unwrap().0;
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let layouts = use_context::<RW<HtmlLayouts>>().unwrap().0;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    view! {
        <Button
            alt=|| "Generate base64 and copy to clipboard".to_string()
            label=view! { {Tabler::OUTLINE_COPY.to_leptos()} }
            action=move || {
                {
                    let settings = SSSsetings::from_context(settings, themes, layouts);
                    match settings.to_base64() {
                        Ok(e) => {
                            spawn_local(async move {
                                let navigator = web_sys::window().unwrap().navigator();
                                let clipboard = navigator.clipboard();
                                if let Err(e) = JsFuture::from(clipboard.write_text(&e)).await {
                                    web_sys::console::error_1(&e);
                                }
                            });
                            store
                                .update(|x| {
                                    x
                                        .push(
                                            ToastContext::Info(
                                                "Your configuration has saved into clipboard!\nPaste it in sss-cli :)"
                                                    .to_string(),
                                            ),
                                        )
                                });
                        }
                        Err(e) => {
                            store
                                .update(|x| {
                                    x
                                        .push(
                                            ToastContext::Error("Got error, check console".to_string()),
                                        )
                                });
                            web_sys::console::error_1(&JsValue::from_str(&e.to_string()))
                        }
                    }
                }
            }
        />
    }
}
