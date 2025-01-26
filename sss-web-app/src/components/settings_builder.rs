use crate::components::reusable_components::prelude::*;
use leptos::prelude::*;
use sss_core::{
    types::{
        link::Link,
        nickname::Nickname,
        skill::{Project, Skill},
    },
    Settings,
};
use sss_std::themes::Themes;

use crate::RW;

#[component]
pub fn SettingsBuilder() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full h-full max-h-dvh gap-4 overflow-y-scroll will-change-scroll">
            <UserSection/>
            <AboutSection/>
            <RepositoriesSection/>
            <SocialsSection/>
            <SkillsSection/>
        </div>
    }
}

#[component]
pub fn UserSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();
    view! {
        <Section title="User section">
            <Stack title="name">
                <Input
                    action=move |ev| {
                        set_settings.update(|x| x.user.name = ev.target().value());
                    }
                    prop=move || {settings.get().user.name}
                />
            </Stack>

            <Stack title="current name">
                <ScrollableBox>
                     <Input
                         action=move |ev| {
                             set_settings.update(|x| x.user.current_nickname.word = ev.target().value());
                         }
                         prop=move || settings.get().user.current_nickname.word
                     />
                     <Input
                         action=move |ev| {
                             set_settings.update(|x| x.user.current_nickname.pronounce = ev.target().value());
                         }
                         prop=move || settings.get().user.current_nickname.pronounce
                     />
                 </ScrollableBox>
            </Stack>

            <Stack title="prevision names">
                <ScrollXBar>
                    {move || (0..settings.read().user.prevision_nicknames.len()).map(move |n| {
                        view! {
                           <ScrollableBox>
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.user.prevision_nicknames[n].word = ev.target().value());
                                    }
                                    prop=move || settings.read().user.prevision_nicknames[n].word.clone()
                                />
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.user.prevision_nicknames[n].pronounce = ev.target().value());
                                    }
                                    prop=move || settings.read().user.prevision_nicknames[n].pronounce.clone()
                                />
                                <Button
                                    label="x"
                                    action=move || {
                                        set_settings.update(|s| {s.user.prevision_nicknames.remove(n);});
                                    }
                                />
                            </ScrollableBox>
                        }
                    }).collect_view()}
                    <Button
                        label="+"
                        action=move || {
                            set_settings.update(|s| s.user.prevision_nicknames.push(Nickname::default()));
                        }
                    />
                </ScrollXBar>
            </Stack>
        </Section>
    }
}

#[component]
pub fn AboutSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();

    view! {
        <Section title="About section">
           <Stack title="about">
                <TextArea
                    action=move |ev| {
                        set_settings.update(|x| x.about = ev.target().value());
                    }
                    prop=move || {settings.get().about}
                />
            </Stack>
            <Stack title="specifications">
                <ScrollXBar>
                    {move || (0..settings.read().specifications.len()).map(move |n| {
                        view! {
                            <div class="grid gap-2 snap-start ">
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|x| x.specifications[n] = ev.target().value());
                                    }
                                    prop=move || settings.read().specifications[n].clone()
                                />
                                <Button
                                    label="x"
                                    action=move || {
                                        set_settings.update(|x| {x.specifications.remove(n);});
                                    }
                                />
                            </div>
                        }
                    }).collect_view()}
                    <Button
                        label="+"
                        action=move || {
                            set_settings.update(|x| x.specifications.push(String::new()));
                        }
                    />
                </ScrollXBar>
            </Stack>
        </Section>
    }
}

#[component]
pub fn RepositoriesSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();

    view! {
        <Section title="Projects section">
            <Stack title="repositories">
                <ScrollXBar>
                    {move || (0..settings.read().repos.len()).map(move |n| {
                        view! {
                            <ScrollableBox>
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.repos[n].name = ev.target().value());
                                    }
                                    prop=move || settings.read().repos[n].name.clone()
                                />
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.repos[n].link.link = ev.target().value());
                                    }
                                    prop=move || settings.read().repos[n].link.link.clone()
                                />
                                <IconSelector
                                    action=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse() {
                                            set_settings.update(|s| s.repos[n].link.provider = value);
                                        }
                                    }
                                    prop=move || settings.read().repos[n].link.provider.to_string()
                                />
                                <Button
                                    label="x"
                                    action=move || {
                                        set_settings.update(|s| { s.repos.remove(n); });
                                    }
                                />
                            </ScrollableBox>
                        }
                    }).collect_view()}
                    <Button
                        label="+"
                        action=move || {
                            set_settings.update(|s| s.repos.push(Project::default()));
                        }
                    />
                </ScrollXBar>
            </Stack>
        </Section>
    }
}

#[component]
pub fn SocialsSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();

    view! {
        <Section title="Social section">
            <Stack title="social sites">
                <ScrollXBar>
                    {move || (0..settings.read().socials.len()).map(move |n| {
                        view! {
                            <ScrollableBox>
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.socials[n].link = ev.target().value());
                                    }
                                    prop=move || settings.read().socials[n].link.clone()
                                />
                                <IconSelector
                                    action=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse() {
                                            set_settings.update(|s| s.socials[n].provider = value);
                                        }
                                    }
                                    prop=move || settings.read().socials[n].provider.to_string()
                                />
                                <Button
                                    label="x"
                                    action=move || {
                                        set_settings.update(|s| { s.socials.remove(n); });
                                    }
                                />
                            </ScrollableBox>
                        }
                    }).collect_view()}
                    <Button
                        label="+"
                        action=move || {
                            set_settings.update(|s| s.socials.push(Link::default()));
                        }
                    />
                </ScrollXBar>
            </Stack>
        </Section>
    }
}

#[component]
pub fn SkillsSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();
    let themes = use_context::<RW<Themes>>().unwrap().0;

    view! {
        <Section title="Skills section">
            <Stack title="skills">
                <ScrollXBar>
                    {move || (0..settings.read().skills.len()).map(move |n| {
                        view! {
                            <ScrollableBox>
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.skills[n].skill = ev.target().value());
                                    }
                                    prop=move || settings.read().skills[n].skill.clone()
                                />

                                <div class="flex gap-2">
                                    <Input
                                        action=move |ev| {
                                            set_settings.update(|s| s.skills[n].since.start = ev.target().value().parse().unwrap_or_default());
                                        }
                                        prop=move || settings.read().skills[n].since.start.to_string()
                                    />
                                    <Input
                                        action=move |ev| {
                                            set_settings.update(|s| s.skills[n].since.end = ev.target().value().parse().unwrap_or_default());
                                        }
                                        prop=move || settings.read().skills[n].since.end.to_string()
                                    />
                                </div>

                                <ScrollableBox>
                                    <Input
                                        action=move |ev| {
                                            set_settings.update(|s| s.skills[n].repo_link.link = ev.target().value());
                                        }
                                        prop=move || settings.read().skills[n].repo_link.link.clone()
                                    />
                                    <IconSelector
                                        action=move |ev| {
                                            if let Ok(value) = event_target_value(&ev).parse() {
                                                set_settings.update(|s| s.skills[n].repo_link.provider = value);
                                            }
                                        }
                                        prop=move || settings.read().skills[n].repo_link.provider.to_string()
                                    />
                                </ScrollableBox>

                                <Stack title="projects">
                                    {move || (0..settings.read().skills[n].projects.len()).map(move |pn| {
                                        view! {
                                            <ScrollableBox>
                                                <Input
                                                    action=move |ev| {
                                                        set_settings.update(|s| s.skills[n].projects[pn].name = ev.target().value());
                                                    }
                                                    prop=move || settings.read().skills[n].projects[pn].name.clone()
                                                />
                                                <Input
                                                    action=move |ev| {
                                                        set_settings.update(|s| s.skills[n].projects[pn].link.link = ev.target().value());
                                                    }
                                                    prop=move || settings.read().skills[n].projects[pn].link.link.clone()
                                                />
                                                <IconSelector
                                                    action=move |ev| {
                                                        if let Ok(value) = event_target_value(&ev).parse() {
                                                            set_settings.update(|s| s.skills[n].projects[pn].link.provider = value);
                                                        }
                                                    }
                                                    prop=move || settings.read().skills[n].projects[pn].link.provider.to_string()
                                                />
                                                <Button
                                                    label="x"
                                                    action=move || {
                                                        set_settings.update(|s| { s.skills[n].projects.remove(pn); });
                                                    }
                                                />
                                            </ScrollableBox>
                                        }
                                    }).collect_view()}
                                    <Button
                                        label="+"
                                        action=move || {
                                            set_settings.update(|s| s.skills[n].projects.push(Project::default()));
                                        }
                                    />
                                </Stack>

                                <button
                                    on:click=move |_| {
                                        set_settings.update(|s| s.skills[n].main = !s.skills[n].main);
                                    }
                                    class="border"
                                    style=move || {
                                        if !settings.read().skills[n].main {
                                            format!(
                                                "background-color: {}; color: {}; border-color: {};",
                                                themes.get().colors().secondary,
                                                themes.get().colors().primary,
                                                themes.get().colors().primary
                                            )
                                        } else {
                                            format!(
                                                "background-color: {}; color: {}; border-color: {};",
                                                themes.get().colors().primary,
                                                themes.get().colors().secondary,
                                                themes.get().colors().secondary
                                            )
                                        }
                                    }
                                >
                                    main
                                </button>

                                <Button
                                    label="x"
                                    action=move || {
                                        set_settings.update(|s| { s.skills.remove(n); });
                                    }
                                />
                            </ScrollableBox>
                        }
                    }).collect_view()}
                    <Button
                        label="+"
                        action=move || {
                            set_settings.update(|s| {
                                s.skills.push(Skill::default());
                                if let Some(e) = s.skills.last_mut() {
                                    e.projects.push(Project::default());
                                }
                            });
                        }
                    />
                </ScrollXBar>
            </Stack>
        </Section>
    }
}
