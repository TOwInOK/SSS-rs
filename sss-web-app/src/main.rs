use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, window};

/// Launch trunk instance
pub fn trunk_launch() {
    use leptos::mount::mount_to_body;
    use sss_web_app::App;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    // Mount body
    mount_to_body(App);

    wasm_bindgen_futures::spawn_local(async {
        let window = window().expect("not found window");
        let promise = JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 100)
                .unwrap();
        }));
        let _ = promise.await;

        if let Some(loader) = window
            .document()
            .expect("no document")
            .query_selector("body > div:first-child")
            .ok()
            .flatten()
        {
            // Добавляем анимацию исчезновения
            loader
                .set_attribute(
                    "style",
                    "
                    position: absolute;
                    top: 0;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    background-color: #1a1a1a;
                    color: white;
                    width: 100%;
                    height: 100vh;
                    font-size: 1.2rem;
                    z-index: 9999;
                    opacity: 1;
                    transition: opacity 0.5s ease-out;
                ",
                )
                .ok();

            // Запускаем анимацию
            loader
                .set_attribute(
                    "style",
                    "
                    position: absolute;
                    top: 0;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    background-color: #1a1a1a;
                    color: white;
                    width: 100%;
                    height: 100vh;
                    font-size: 1.2rem;
                    z-index: 9999;
                    opacity: 0;
                    transition: opacity 0.5s ease-out;
                ",
                )
                .ok();

            let promise = JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve, 500, // Время анимации
                    )
                    .unwrap();
            }));
            let _ = promise.await;

            if let Some(parent) = loader.parent_node() {
                parent.remove_child(&loader).ok();
            }
        }
    });
}

fn main() {
    trunk_launch()
}
