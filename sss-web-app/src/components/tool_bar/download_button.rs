use crate::{
    RW,
    components::{
        reusable_components::button::Button,
        toster::{ToastContext, ToastStore},
    },
};

use leptos::{
    IntoView,
    prelude::{Get, Update},
    view,
    wasm_bindgen::JsValue,
};
use leptos::{component, task::spawn_local};
use leptos::{prelude::use_context, web_sys};
use render::render::Render;
use sss_core::{Data, types::provider::Tabler};
use sss_std::{prelude::*, themes::Themes};
use web_sys::{Blob, Url, js_sys};

#[component]
pub fn DownloadButton() -> impl IntoView {
    let settings = use_context::<RW<Data>>().unwrap().0;
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let layouts = use_context::<RW<HtmlLayouts>>().unwrap().0;
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    let download_handler = move || {
        let data = settings.get();
        let theme = themes.get().into();
        let layout = layouts.get();
        let render = HtmlTeraRender::new(&data, theme, &layout);
        match render.finalize(&DefaultTemplates::STANDART).render() {
            Ok(html_content) => {
                let window = web_sys::window().expect("no global window exists");

                let arr = js_sys::Array::new();
                arr.push(&JsValue::from_str(&html_content));

                let blob = Blob::new_with_str_sequence(&arr).expect("should create blob");

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
                    x.push(ToastContext::Info(
                        "HTML has pushed on your PC!".to_string(),
                    ))
                });

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
        <Button
            alt=|| "Download html with card".to_string()
            action=download_handler label=view! {
            {Tabler::OUTLINE_HTML.to_leptos()}
        }/>
    }
}
