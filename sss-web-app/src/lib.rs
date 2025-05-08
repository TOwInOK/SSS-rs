use codee::string::JsonSerdeCodec;
use components::toster::ToastStore;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_use::storage::{UseStorageOptions, use_local_storage_with_options};
use pages::home::HomePage;

use sss_core::Data;
use sss_std::prelude::{HtmlLayouts, Themes};
use tools::gen_example_config;

// Modules
pub mod components;
mod pages;
pub mod tools;
// Top-Level pages
use crate::pages::card_editor::CardEditor;

pub type RW<T> = (Signal<T>, WriteSignal<T>);
pub type M<T> = Memo<T>;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (settings, set_settings, _) = use_local_storage_with_options::<Data, JsonSerdeCodec>(
        "settings",
        UseStorageOptions::default().initial_value(gen_example_config()),
    );
    let (themes, set_themes, _) = use_local_storage_with_options::<Themes, JsonSerdeCodec>(
        "themes",
        UseStorageOptions::default().initial_value(Themes::ROSE_PINE),
    );
    let (layouts, set_layouts, _) = use_local_storage_with_options::<HtmlLayouts, JsonSerdeCodec>(
        "layouts",
        UseStorageOptions::default().initial_value(HtmlLayouts::UMBRELLA),
    );

    let toaster_store = signal(ToastStore::default());

    provide_context((settings, set_settings));
    provide_context((themes, set_themes));
    provide_context((layouts, set_layouts));
    provide_context::<RW<ToastStore>>((toaster_store.0.into(), toaster_store.1));

    view! {
        <Html attr:lang="en" attr:dir="ltr" />

        <Title text="SSS-rs test" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="darkreader" content="none" />
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
        <link href=move || themes.read().font().1 rel="stylesheet" />
        <Style>
            {move || {
                format!(
                    r#":root {{--color-text: {}; --color-background: {}; --color-accent: {}; --color-border: {};}}"#,
                    themes.read().colors().text,
                    themes.read().colors().background,
                    themes.read().colors().accent,
                    themes.read().colors().border,
                )
            }}
        </Style>

        <Router>
            <div
                style=move || {
                    format!(
                        "background-color: {}; color: {}",
                        themes.read().colors().background,
                        themes.read().colors().text,
                    )
                }
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
