use std::time::Duration;

use leptos::prelude::*;
use sss_std::themes::Themes;

use crate::{components::reusable_components::section::Section, RW};
#[inline]
pub fn toast_info(context: impl ToString) {
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    store.update(|x| x.push(ToastContext::Info(context.to_string())));
}
#[inline]
pub fn toast_error(context: impl ToString) {
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    store.update(|x| x.push(ToastContext::Error(context.to_string())));
}
#[inline]
pub fn toast_warn(context: impl ToString) {
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .1;
    store.update(|x| x.push(ToastContext::Warn(context.to_string())));
}

#[derive(Debug, Clone)]
pub struct ToastStore {
    pub contexts: Vec<ToastContext>,
}

impl Default for ToastStore {
    fn default() -> Self {
        Self::new()
    }
}
impl ToastStore {
    pub fn new() -> Self {
        Self { contexts: vec![] }
    }
    pub fn push(&mut self, context: ToastContext) {
        self.contexts.push(context);
    }

    pub fn pop(&mut self) -> Option<ToastContext> {
        self.contexts.pop()
    }
}

#[component]
pub fn ToastStore() -> impl IntoView {
    let store = use_context::<RW<ToastStore>>()
        .expect("ToastStore should be provided")
        .0;

    view! {
        <div class="w-full flex" style={move ||
            if !store.read().contexts.is_empty() {
                "opacity: 100;"
            } else {
                "opacity: 0;"
            }
        }>
            <Section title="Notifications">
                <For
                    each= move || store.read().contexts.clone().into_iter().enumerate()
                    key=|(a,b)| format!("Toast-{}-{}",a, b.title())
                    let:context
                >
                    <Toast context=context.1.clone()/>
                </For>
            </Section>
        </div>
    }
}

#[component]
fn Toast(context: ToastContext) -> impl IntoView {
    let (bg, fg) = context.colors();
    let store = use_context::<RW<ToastStore>>().unwrap().1;
    let context_clone = context.clone();
    set_timeout(
        move || {
            store.update(|s| {
                s.contexts.retain(|x| x != &context_clone);
            });
        },
        Duration::from_secs(5),
    );
    view! {
        <div class="flex flex-col p-1.5 border gap-4 w-full overflow-clip z-20"
            style=format!("background-color: {};", bg)
        >
            <p class="w-full pl-2 font-bold"
                style=format!("background-color: {}; color: {}", fg, bg)
            >
                {context.title()}
            </p>
            {context.inner()}
        </div>
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ToastContext {
    Info(String),
    Error(String),
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
    pub fn inner(&self) -> String {
        match self {
            ToastContext::Info(e) => e.clone(),
            ToastContext::Error(e) => e.clone(),
            ToastContext::Warn(e) => e.clone(),
        }
    }

    pub fn colors(&self) -> (&'static str, &'static str) {
        let colors = use_context::<RW<Themes>>().unwrap().0.read();
        let colors = colors.colors();
        match self {
            ToastContext::Info(_) => (colors.secondary, colors.primary),
            ToastContext::Error(_) => (colors.secondary, colors.primary),
            ToastContext::Warn(_) => (colors.secondary, colors.primary),
        }
    }
}
