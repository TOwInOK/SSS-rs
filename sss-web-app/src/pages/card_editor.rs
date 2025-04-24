use leptos::prelude::*;

use crate::components::card_viewer::CardViewer;
use crate::components::reusable_components::selector::{LayoutSelector, ThemeSelector};
use crate::components::settings_builder::SettingsBuilder;
use crate::components::tool_bar::ToolBar;
use crate::components::toster::ToastStore;

#[component]
pub fn CardEditor() -> impl IntoView {
    view! {
        <div class="grid grid-row-2 md:grid-row-1 md:grid-cols-2 md:gap-4 md:grid-cols-[1fr_1fr] ">
            <SettingsBuilder />
            // card view and selectors
            <div class="flex-col h-screen overflow-y-auto  md:h-full inline-flex md:gap-4 order-first md:order-last relative">
                <div class="grid 2xl:grid-cols-3 gap-4 p-1.5 -pl-1.5 border">
                    <ThemeSelector />
                    <ToolBar />
                    <LayoutSelector />
                </div>
                <CardViewer />
                <ToastStore />
            </div>
        </div>
    }
}
