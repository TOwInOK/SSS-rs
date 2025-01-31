use leptos::{html::Div, prelude::*};
use render::{layout::Layout, theme::Shade};
use sss_core::Settings;
use sss_std::{prelude::Layouts, themes::Themes, tools::gen_css};

use crate::RW;

/// Компонент для отображения сгенерированной карточки на основе настроек.
#[component]
pub fn CardViewer() -> impl IntoView {
    let settings = use_context::<RW<Settings>>().unwrap().0;
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let layouts = use_context::<RW<Layouts>>().unwrap().0;

    let card_node = NodeRef::<Div>::new();

    let rendered_card = Memo::new(move |_| {
        let theme = themes.get().into();
        let layout = layouts.get();

        layout
            .to_layout(&settings.read(), theme)
            .render()
            .map(|card| {
                let css = gen_css(&theme.get_encre_css_config(), &card);
                format!(r#"<style>{css}</style>{card}"#)
            })
            .ok()
    });

    card_node.on_load(move |el| {
        Effect::new(move |_| {
            if let Some(content) = rendered_card.get() {
                el.set_inner_html(&content);
            }
        });
    });

    view! {
        <div class="relative h-full w-full p-4 content-center justify-center items-center grid place-items-center">
            // Background grid
            <div style=move || {
                let color = themes.get().colors().primary;
                format!(
                    r#"position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background-image:
                        linear-gradient(to right, {color}80 1px, transparent 1px),
                        linear-gradient(to bottom, {color}60 1px, transparent 1px);
                    background-size: 32px 32px;
                    pointer-events: none;
                    z-index: 0;
                    "#,
                )
            }></div>

            // Card
            <div
                class="inline-flex overflow-hidden relative z-10 will-change-transform transition duration-300 ease-in transition-discrete transition-all rounded-xl drop-shadow-xl shadow-xl"
                node_ref=card_node
            ></div>
        </div>
    }
}
