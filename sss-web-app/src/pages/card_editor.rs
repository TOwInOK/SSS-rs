use leptos::prelude::*;

use crate::components::card_viewer::CardViewer;
use crate::components::reusable_components::button::{
    DownloadButton, DropButton, LoadButton, RestoreButton, SaveButton,
};
use crate::components::reusable_components::selector::{LayoutSelector, ThemeSelector};
use crate::components::settings_builder::SettingsBuilder;
use crate::components::toster::ToastStore;

#[component]
pub fn CardEditor() -> impl IntoView {
    view! {
        <div class="grid grid-row-2 md:grid-row-1 md:grid-cols-2 gap-4 md:grid-cols-[1fr_1fr] ">
            <SettingsBuilder/>
            // card view and selectors
            <div class="flex-col inline-flex gap-4 order-first md:order-last relative">
                <div class="grid lg:grid-cols-3 gap-4 p-1.5 -pl-1.5 border">
                    <ThemeSelector />
                    <div class="flex border p-1.5 justify-evenly md:gap-0">
                        <DropButton/>
                        <RestoreButton/>
                        <SaveButton/>
                        <LoadButton/>
                        <DownloadButton/>
                    </div>
                    <LayoutSelector />
                </div>
                <CardViewer />
                <ToastStore/>
            </div>
        </div>

    }
}
