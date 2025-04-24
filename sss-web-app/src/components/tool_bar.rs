use download_button::DownloadButton;
use drop_button::DropButton;
use leptos::prelude::*;
use load_button::LoadButton;
use restore_button::RestoreButton;
use save_button::SaveButton;

pub mod download_button;
pub mod drop_button;
pub mod load_button;
pub mod restore_button;
pub mod save_button;

#[component]
pub fn ToolBar() -> impl IntoView {
    view! {
        <div class="flex border p-1.5 justify-evenly md:gap-0">
            <DropButton />
            <RestoreButton />
            <SaveButton />
            <LoadButton />
            <DownloadButton />
        </div>
    }
}
