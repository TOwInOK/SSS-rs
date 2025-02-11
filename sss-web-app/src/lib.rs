use codee::string::JsonSerdeCodec;
use components::toster::ToastStore;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_use::storage::use_local_storage;
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

    let (settings_store, settings_set_store, _) =
        use_local_storage::<Settings, JsonSerdeCodec>("settings");
    let (themes_store, themes_set_store, _) = use_local_storage::<Themes, JsonSerdeCodec>("themes");
    let (layouts_store, layouts_set_store, _) =
        use_local_storage::<Layouts, JsonSerdeCodec>("layouts");

    let settings = signal({
        if is_local_storage_settings_exist() {
            settings_store
                .try_get_untracked()
                .map_or_else(gen_example_config, |x| x)
        } else {
            gen_example_config()
        }
    });
    let themes = signal(themes_store.get_untracked());
    let layouts = signal(layouts_store.get_untracked());
    let toster_store = signal(ToastStore::default());

    provide_context(themes);
    provide_context(settings);
    provide_context(layouts);
    provide_context(toster_store);

    Effect::watch(
        move || settings.0.get(),
        move |current, _, _| settings_set_store.set(current.clone()),
        true,
    );
    Effect::watch(
        move || themes.0.get(),
        move |current, _, _| themes_set_store.set(current.clone()),
        true,
    );
    Effect::watch(
        move || layouts.0.get(),
        move |current, _, _| layouts_set_store.set(current.clone()),
        true,
    );

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

fn is_local_storage_settings_exist() -> bool {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("settings").ok())
        .flatten()
        .is_some()
}
