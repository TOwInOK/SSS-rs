use std::time::Duration;

use leptos::prelude::*;
use sss_std::themes::Themes;
use web_sys::js_sys::Date;

use crate::{
    RW,
    components::reusable_components::{button::Button, section::Section},
};
/// Отображает информационное сообщение с заданным контекстом.
#[inline]
pub fn toast_info(context: impl ToString) {
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    store.update(|x| x.push(ToastContext::Info(context.to_string())));
}
/// Отображает сообщение об ошибке с заданным контекстом.
#[inline]
pub fn toast_error(context: impl ToString) {
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    store.update(|x| x.push(ToastContext::Error(context.to_string())));
}
/// Отображает предупреждающее сообщение с заданным контекстом.
#[inline]
pub fn toast_warn(context: impl ToString) {
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    store.update(|x| x.push(ToastContext::Warn(context.to_string())));
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastItem {
    context: ToastContext,
    id: usize,
    created_at: f64,
}

#[derive(Debug, Clone)]
pub struct ToastStore {
    pub items: Vec<ToastItem>,
    next_id: usize,
}

impl Default for ToastStore {
    fn default() -> Self {
        Self::new()
    }
}
impl ToastStore {
    pub fn new() -> Self {
        Self {
            items: vec![],
            next_id: 0,
        }
    }
    pub fn push(
        &mut self,
        context: ToastContext,
    ) {
        let toast_item = ToastItem {
            context,
            id: self.next_id,
            created_at: Date::now(),
        };
        self.items.push(toast_item);
        self.next_id += 1;
    }

    pub fn remove(
        &mut self,
        id: usize,
    ) {
        self.items.retain(|item| item.id != id);
    }
}

/// Компонент для отображения всех уведомлений.
#[component]
pub fn ToastStore() -> impl IntoView {
    let store = use_context::<RW<ToastStore>>().expect("ToastStore should be provided");

    Effect::new(move |_| {
        let store_signal = store.1;
        set_interval(
            move || {
                store_signal.update(|store| {
                    let now = Date::now();
                    store.items.retain(|item| (now - item.created_at) < 5000.0);
                });
            },
            Duration::from_millis(100),
        );
    });

    view! {
        <div class="fixed bottom-0 right-0 grid gap-2 z-50 transition-discrete transition-all will-change-transform will-change-opacity will-change-contents ease-in-out duration-300" style={move ||
            if !store.0.read().items.is_empty() {
                "opacity: 100; transform: translateY(0);"
            } else {
                "opacity: 0; transform: translateY(100%);"
            }
        }>
            <Section title="Notifications">
                <For
                    each=move || store.0.read().items.clone()
                    key=|item| item.id
                    let:item
                >
                    <Toast
                        context=item.context.clone()
                        id=item.id
                    />
                </For>
            </Section>
        </div>
    }
}

/// Компонент для отображения одного уведомления.
#[component]
fn Toast(
    context: ToastContext,
    id: usize,
) -> impl IntoView {
    let store = use_context::<RW<ToastStore>>().unwrap().1;
    let context = signal(context).0;

    view! {
        <div
            id={id}
            class="grid gap-4 p-1.5 border z-50"
            style=move || format!("background-color: {};", context.get().bg())
        >
            <div class="grid grid-cols-[5fr_1fr] gap-4">
                <p class="pl-2 font-bold w-full"
                    style=move || format!("background-color: {}; color: {}", context.get().fg(), context.get().bg())
                >
                    {move || context.get().title()}
                </p>
                <Button
                    alt=|| "Close toast".to_string()
                    label="X"
                    action=move || {store.update(|s| s.remove(id));}
                />
            </div>
            {move || context.get().inner()}
        </div>
    }
}
/// Тип сообщения уведомления.
#[derive(Debug, Clone, PartialEq)]
pub enum ToastContext {
    /// Информационное сообщение.
    Info(String),
    /// Сообщение об ошибке.
    Error(String),
    /// Предупреждающее сообщение.
    Warn(String),
}

impl ToastContext {
    pub fn title(&self) -> &'static str {
        match self {
            ToastContext::Info(_) => "Info",
            ToastContext::Error(_) => "Error",
            ToastContext::Warn(_) => "Warn",
        }
    }
    /// Return text of toast
    pub fn inner(&self) -> String {
        match self {
            ToastContext::Info(e) => e.clone(),
            ToastContext::Error(e) => e.clone(),
            ToastContext::Warn(e) => e.clone(),
        }
    }

    /// return colors of toast
    pub fn bg(&self) -> &'static str {
        let colors = use_context::<RW<Themes>>().unwrap().0.read();
        let colors = colors.colors();
        match self {
            ToastContext::Info(_) => colors.background,
            ToastContext::Error(_) => colors.background,
            ToastContext::Warn(_) => colors.background,
        }
    }
    /// return colors of toast
    pub fn fg(&self) -> &'static str {
        let colors = use_context::<RW<Themes>>().unwrap().0.read();
        let colors = colors.colors();
        match self {
            ToastContext::Info(_) => colors.text,
            ToastContext::Error(_) => colors.accent,
            ToastContext::Warn(_) => colors.accent,
        }
    }
}
