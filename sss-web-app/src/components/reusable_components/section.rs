use crate::{
    RW,
    components::reusable_components::text::{Text, TextStyle},
};
use leptos::prelude::*;
use sss_std::themes::Themes;
#[component]
/// Section with items
pub fn SectionInverted(
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <BlankedSection>
            <Text title={title} inverted=true style=crate::components::reusable_components::text::TextStyle::Primary/>
            {children()}
        </BlankedSection>
    }
}

#[component]
/// Section with items
pub fn Section(
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <BlankedSection>
            <Text title={title} inverted=true style=crate::components::reusable_components::text::TextStyle::Primary/>
            {children()}
        </BlankedSection>
    }
}

#[component]
pub fn SectionInvertedWith(
    title: &'static str,
    children: Children,
    with: impl IntoView + 'static,
) -> impl IntoView {
    view! {
        <BlankedSection>
            <div class="grid grid-cols-[5fr_1fr] gap-4">
                <Text title={title} inverted=true style=TextStyle::Primary/>
                {with}
            </div>
            {children()}
        </BlankedSection>
    }
}

#[component]
pub fn SectionWith(
    title: &'static str,
    children: Children,
    with: impl IntoView + 'static,
) -> impl IntoView {
    view! {
        <BlankedSection>
            <div class="grid grid-cols-[5fr_1fr] gap-4">
                <Text title={title} style=TextStyle::Primary/>
                {with}
            </div>
            {children()}
        </BlankedSection>
    }
}

#[component]
pub fn BlankedSection(children: Children) -> impl IntoView {
    let themes = use_context::<RW<Themes>>().unwrap().0;
    view! {
        <div class="grid gap-4 p-4 border"
            style=move || format!(
                "background-color: {};",
                themes.get().colors().background
            )
        >
            {children()}
        </div>
    }
}
