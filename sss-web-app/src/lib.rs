use components::toster::ToastStore;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use pages::home::HomePage;
use sss_core::Settings;
use sss_std::{prelude::Layouts, themes::Themes};
use tools::gen_example_config;
// Modules
pub mod components;
mod pages;
pub mod tools;
// Top-Level pages
use crate::pages::card_editor::CardEditor;

pub type RW<T> = (ReadSignal<T>, WriteSignal<T>);

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let themes = signal(Themes::ROSE_PINE);
    let settings = signal(Settings::default());
    let layouts = signal(Layouts::default());
    let toster_store = signal(ToastStore::default());

    Effect::new(move || {});

    provide_context(themes);
    provide_context(settings);
    provide_context(layouts);
    provide_context(toster_store);

    settings.1.update(|x| *x = gen_example_config());
    view! {
        <Html attr:lang="en" attr:dir="ltr" />

        <Title text="SSS-rs test" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta name="darkreader" content="none"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
        <link href={move || themes.0.read().font().1} rel="stylesheet" />


            <Router>
                <div
                    style=move || {format!("background-color: {}; color: {}", themes.0.read().colors().secondary, themes.0.read().colors().primary)}
                    class="grid min-h-dvh overflow-y-scroll overflow-hidden transition duration-300 ease-in transition-discrete transition-all"
                >
                    <Routes fallback=|| view! { NotFound }>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/editor") view=CardEditor />
                    </Routes>

                </div>
            </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[cfg(feature = "ssr")]
mod ssr_imports {
    use crate::{shell, App};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use log::{info, Level};
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub struct Handler(axum_js_fetch::App);

    #[wasm_bindgen]
    impl Handler {
        pub async fn new() -> Self {
            _ = console_log::init_with_level(Level::Debug);
            console_error_panic_hook::set_once();

            let leptos_options = LeptosOptions::builder()
                .output_name("client")
                .site_pkg_dir("pkg")
                .build();

            let routes = generate_route_list(App);

            // build our application with a route
            let app = Router::new()
                .leptos_routes(&leptos_options, routes, {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                })
                .with_state(leptos_options);

            info!("creating handler instance");
            let r = axum_js_fetch::App::new(app);
            Self(r)
        }

        pub async fn serve(&self, req: web_sys::Request) -> web_sys::Response {
            self.0.oneshot(req).await
        }
    }
}
