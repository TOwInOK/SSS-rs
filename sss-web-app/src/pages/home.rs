use crate::components::{card_viewer::CardViewer, reusable_components::prelude::*};
use leptos::prelude::*;
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="grid md:grid-cols-2 gap-8 md:gap-20 lg:gap-30 p-4 md:p-8 max-w-7xl mx-auto items-center justify-center">
            // Left Panel - Information
            <div class="inline-flex flex-col gap-8">
                <Section title="SSS-rs (Skill, Slick, Style)">
                    <div class="space-y-6">
                        // Hero Section
                        <Stack title="About project">
                            <p class="text-lg opacity-90">
                                "Modern toolkit for creating stunning developer profile cards.
                                Customize layouts, themes and share your professional identity."
                            </p>
                        </Stack>

                        // Feature Icons
                        <Stack title="Features">
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                                <FeatureCard
                                    icon=view! {
                                        <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-brush"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M3 21v-4a4 4 0 1 1 4 4h-4" /><path d="M21 3a16 16 0 0 0 -12.8 10.2" /><path d="M21 3a16 16 0 0 1 -10.2 12.8" /><path d="M10.6 9a9 9 0 0 1 4.4 4.4" /></svg>
                                    }
                                    title="Themes"
                                    text="Choose from beautiful color schemes"
                                />
                                <FeatureCard
                                    icon=view! {
                                        <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-layout"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M4 4m0 2a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2z" /><path d="M4 13m0 2a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2z" /><path d="M14 4m0 2a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2z" /></svg>
                                    }
                                    title="Layouts"
                                    text="Multiple card arrangement options"
                                />
                                <FeatureCard
                                    icon=view! {
                                        <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-tools"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M3 21h4l13 -13a1.5 1.5 0 0 0 -4 -4l-13 13v4" /><path d="M14.5 5.5l4 4" /><path d="M12 8l-5 -5l-4 4l5 5" /><path d="M7 8l-1.5 1.5" /><path d="M16 12l5 5l-4 4l-5 -5" /><path d="M16 17l-1.5 1.5" /></svg>
                                    }
                                    title="CLI Tool"
                                    text="Generate cards from terminal"
                                />
                                <FeatureCard
                                    icon=view!{
                                        <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-rocket"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M4 13a8 8 0 0 1 7 7a6 6 0 0 0 3 -5a9 9 0 0 0 6 -8a3 3 0 0 0 -3 -3a9 9 0 0 0 -8 6a6 6 0 0 0 -5 3" /><path d="M7 14a6 6 0 0 0 -3 6a6 6 0 0 0 6 -3" /><path d="M15 9m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /></svg>
                                    }
                                    title="Live Preview"
                                    text="Instant changes visualization"
                                />
                            </div>
                        </Stack>

                        // Quick Links
                        <Stack title="Links">
                            <div class="flex gap-4 flex-wrap">
                                <IconLink
                                    href="https://github.com/TOwInOK/SSS-rs"
                                    text="Project"
                                />
                                <IconLink
                                    href="https://github.com/TOwInOK"
                                    text="Author"
                                />
                            </div>
                        </Stack>

                        // Getting Started
                        <Stack title="How to Use">
                            <div class="grid gap-4">
                                <StepCard
                                    number=1
                                    title="Edit Your Card"
                                    text="Use our intuitive editor to customize your profile"
                                    action=view! { <a href="editor" class="p-2 mt-2 rounded-lg border bg-opacity-10 flex transform hover:scale-102 transition-all duration-600">Go to Editor</a> }
                                />
                                <StepCard
                                    number=2
                                    title="Export & Share"
                                    text="Generate Base64 string or HTML and share anywhere"
                                    action=view! { <a href="editor" class="p-2 mt-2 rounded-lg border bg-opacity-10 flex transform hover:scale-102 transition-all duration-600">Start sharing?</a> }
                                />
                            </div>
                        </Stack>
                    </div>
                </Section>
            </div>

            // Right Panel - Card Preview
            <div class="sticky flex items-center justify-center content-center">
                <div class="inline-flex md:transform-3d lg:rotate-x-51 md:rotate-x-31 md:-rotate-y-10 md:rotate-z-10 lg:rotate-z-20 duration-600 ease-in-out transition-all backface-hidden drop-shadow shadow-lg overflow-hidden p-1.5 md:p-2 lg:p-4">
                    <CardViewer/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn FeatureCard(
    icon: impl IntoView,
    title: impl IntoView,
    text: impl IntoView,
) -> impl IntoView {
    view! {
        <div class="p-4 rounded-lg border bg-opacity-10 flex gap-3 items-start">
            <span class="text-2xl">
                {icon}
            </span>
            <div>
                <h3 class="font-bold text-sm md:text-lg text-pretty">{title}</h3>
                <p class="text-ms md:text-sm opacity-75 text-pretty">{text}</p>
            </div>
        </div>
    }
}

#[component]
fn IconLink(
    href: &'static str,
    text: impl IntoView,
) -> impl IntoView {
    view! {
        <a href={href} class="flex items-center gap-2 px-4 py-2 rounded-lg border transform hover:scale-102 transition-all duration-600">
            <span>{text}</span>
        </a>
    }
}

#[component]
fn StepCard(
    number: impl IntoView,
    title: impl IntoView,
    text: impl IntoView,
    action: impl IntoView,
) -> impl IntoView {
    view! {
        <div class="p-4 rounded-lg border-l-4 border-primary pl-4 bg-gradient-to-r from-transparent to-secondary/5">
            <div class="flex gap-3 items-start">
                <div class="w-8 h-8 rounded-full bg-primary text-secondary flex items-center justify-center font-bold">
                    {number}
                </div>
                <div class="flex-1">
                    <h3 class="font-bold text-lg mb-2">{title}</h3>
                    <p class="text-sm opacity-75 mb-3">{text}</p>
                    {action}
                </div>
            </div>
        </div>
    }
}
