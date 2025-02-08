use leptos::prelude::*;
use sss_std::themes::Themes;

use crate::{
    components::toster::{ToastContext, ToastStore},
    RW,
};

use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsValue;
use leptos::web_sys;
use render::layout::Finalise;
use sss_core::Settings;
use sss_std::prelude::Layouts;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, Blob, Url};

use crate::tools::SSSsetings;

/// Кнопка с заданным `label` и действием `action`.
#[component]
pub fn Button<A: Fn() + 'static>(
    label: impl IntoView + Clone,
    action: A,
) -> impl IntoView {
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let mut css = "border font-bold".to_string();
    if label.clone().to_html() == *"+" {
        css.push_str(" p-4");
    }
    view! {
        <button
            on:click=move |_| {
                action()
            }
            class=css
            style=move || format!(
                "background-color: {}; color: {}",
                themes.get().colors().primary,
                themes.get().colors().secondary
            )
        >{label}</button>
    }
}

/// Кнопка для сброса настроек.
#[component]
pub fn DropButton() -> impl IntoView {
    let set_settings = use_context::<RW<Settings>>().unwrap().1;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;

    view! {
        <Button label=view! {
            <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-trash"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M4 7l16 0" /><path d="M10 11l0 6" /><path d="M14 11l0 6" /><path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" /><path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" /></svg>
        } action= move || {
            set_settings.set(Settings::default());
             store.update(|x| x.push(ToastContext::Info("Configuration has dropped!".to_string())));
        }/>
    }
}

/// Кнопка для сохранения настроек в буфер обмена.
#[component]
pub fn SaveButton() -> impl IntoView {
    let settings = use_context::<RW<Settings>>().unwrap().0;
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let layouts = use_context::<RW<Layouts>>().unwrap().0;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    view! {
        <Button label=view! {
            <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-clipboard-copy"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h3m9 -9v-5a2 2 0 0 0 -2 -2h-2" /><path d="M13 17v-1a1 1 0 0 1 1 -1h1m3 0h1a1 1 0 0 1 1 1v1m0 3v1a1 1 0 0 1 -1 1h-1m-3 0h-1a1 1 0 0 1 -1 -1v-1" /><path d="M9 3m0 2a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v0a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2z" /></svg>
        }
        action= move || {
           {
               let settings = SSSsetings::from_context(settings, themes, layouts);
               match settings.to_base64() {
                   Ok(e) => {
                       spawn_local(async move {
                           let navigator = web_sys::window().unwrap().navigator();
                           let clipboard = navigator.clipboard();
                           if let Err(e) = JsFuture::from(clipboard.write_text(&e)).await{
                               web_sys::console::error_1(&e);
                           }
                       });
                       store.update(|x| x.push(ToastContext::Info("Your configuration has saved into clipboard!\nPaste it in sss-cli :)".to_string())));

                   },
                   Err(e) => {
                       store.update(|x| x.push(ToastContext::Error("Got error, check console".to_string())));
                       web_sys::console::error_1(&JsValue::from_str(&e.to_string()))
                   },
               }

           }
        }/>
    }
}

/// Кнопка для загрузки настроек из буфера обмена.
#[component]
pub fn LoadButton() -> impl IntoView {
    let set_settings = use_context::<RW<Settings>>().unwrap().1;
    let set_themes = use_context::<RW<Themes>>().unwrap().1;
    let set_layouts = use_context::<RW<Layouts>>().unwrap().1;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    view! {
        <Button label=view! {
            <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-clipboard-plus"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /><path d="M9 3m0 2a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v0a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2z" /><path d="M10 14h4" /><path d="M12 12v4" /></svg>
        } action= move || {
           {
               spawn_local(async move {
                   let navigator = web_sys::window().unwrap().navigator();
                   let clipboard = navigator.clipboard();
                   match JsFuture::from(clipboard.read_text()).await {
                        Ok(value) => {
                            if let Some(value) = value.as_string() {
                                let settings = SSSsetings::from_base64(&value);
                                match settings {
                                    Ok(e) => {
                                        store.update(|x| x.push(ToastContext::Info("Your base64 has loaded!".to_string())));
                                        e.update_context(set_settings, set_themes, set_layouts)
                                    },
                                    Err(e) => {
                                        store.update(|x| x.push(ToastContext::Error("Your base64 is't valid".to_string())));
                                        web_sys::console::error_1(&JsValue::from_str(&e.to_string()))
                                    },
                                }
                            }
                        },
                        Err(err) => {
                            store.update(|x| x.push(ToastContext::Error("Got error, check console".to_string())));
                            web_sys::console::error_1(&err);
                        }
                   }

               });
           }
        }/>
    }
}

/// Кнопка для скачивания настроек в виде HTML файла.
#[component]
pub fn DownloadButton() -> impl IntoView {
    let settings = use_context::<RW<Settings>>().unwrap().0;
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let layouts = use_context::<RW<Layouts>>().unwrap().0;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    let download_handler = move || {
        let t = themes.get().into();
        let s = settings.get();
        match layouts.get().to_layout(&s, t).finalize() {
            Ok(html_content) => {
                let window = web_sys::window().expect("no global window exists");

                // 1. Создаем правильный Blob
                let arr = js_sys::Array::new();
                arr.push(&JsValue::from_str(&html_content));

                let blob = Blob::new_with_str_sequence(&arr).expect("should create blob");

                // // 2. Добавляем MIME-тип
                // let mut options = BlobPropertyBag::new();
                // options.type_("text/html");

                // 3. Создаем URL с правильными параметрами
                let url = Url::create_object_url_with_blob(&blob).expect("should create URL");
                use leptos::wasm_bindgen::JsCast;
                // 4. Используем временный <a> элемент для скачивания
                let document = window.document().expect("no document exists");
                let a = document.create_element("a").unwrap();
                let a: web_sys::HtmlAnchorElement = a.dyn_into().unwrap();

                a.set_href(&url);
                a.set_download("generated.html");
                a.click();
                store.update(|x| {
                    x.push(ToastContext::Error(
                        "HTML has pushed on your PC!".to_string(),
                    ))
                });

                // 5. Очистка
                spawn_local(async move {
                    Url::revoke_object_url(&url).unwrap();
                });
            }
            Err(e) => {
                store.update(|x| {
                    x.push(ToastContext::Error("Got error, check console".to_string()))
                });
                web_sys::console::error_1(&JsValue::from_str(&e.to_string()));
            }
        };
    };

    view! {
        <Button action=download_handler label=view! {
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-file-type-html">
                <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                <path d="M14 3v4a1 1 0 0 0 1 1h4" />
                <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" />
                <path d="M2 21v-6" />
                <path d="M5 15v6" />
                <path d="M2 18h3" />
                <path d="M20 15v6h2" />
                <path d="M13 21v-6l2 3l2 -3v6" />
                <path d="M7.5 15h3" />
                <path d="M9 15v6" />
            </svg>
        }/>
    }
}
