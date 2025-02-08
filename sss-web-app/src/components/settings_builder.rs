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

/// Главный компонент настроек, отображает все секции настроек.
#[component]
pub fn SettingsBuilder() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 gap-4 h-full md:max-h-dvh md:overflow-y-scroll md:will-change-scroll">
            <UserSection/>
            <AboutSection/>
            <RepositoriesSection/>
            <SocialsSection/>
            <SkillsSection/>
        </div>
    }
}

/// Компонент для настройки пользовательских данных.
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
                <For
                    each= move || (0..settings.read().user.prevision_nicknames.len())
                    key= |index| *index
                    let:index
                 >
                   <ScrollableBox>
                       <Input
                           action=move |ev| {
                               set_settings.update(|s| s.user.prevision_nicknames[index].word = ev.target().value());
                           }
                           prop=move || settings.read().user.prevision_nicknames[index].word.clone()
                       />
                       <Input
                           action=move |ev| {
                               set_settings.update(|s| s.user.prevision_nicknames[index].pronounce = ev.target().value());
                           }
                           prop=move || settings.read().user.prevision_nicknames[index].pronounce.clone()
                       />
                       <Button
                           label="x"
                           action=move || {
                               set_settings.update(|s| {s.user.prevision_nicknames.remove(index);});
                           }
                       />
                   </ScrollableBox>
                </For>
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

/// Компонент для настройки секции "О себе".
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
                   <For
                       each=move || (0..settings.read().specifications.len())
                       key=|index| *index
                       let:index
                   >
                        <div class="grid gap-2 snap-start ">
                            <Input
                                action=move |ev| {
                                    set_settings.update(|x| x.specifications[index] = ev.target().value());
                                }
                                prop=move || settings.read().specifications[index].clone()
                            />
                            <Button
                                label="x"
                                action=move || {
                                    set_settings.update(|x| {x.specifications.remove(index);});
                                }
                            />
                        </div>
                  </For>
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

/// Компонент для настройки секции "Проекты".
#[component]
pub fn RepositoriesSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();

    view! {
        <Section title="Projects section">
            <Stack title="repositories">
                <ScrollXBar>
                    <For
                        each= move || (0..settings.read().repos.len())
                        key= |index| *index
                        let:index
                    >
                        <ScrollableBox>
                            <Input
                                action=move |ev| {
                                    set_settings.update(|s| s.repos[index].name = ev.target().value());
                                }
                                prop=move || settings.read().repos[index].name.clone()
                            />
                            <Input
                                action=move |ev| {
                                    set_settings.update(|s| s.repos[index].link.link = ev.target().value());
                                }
                                prop=move || settings.read().repos[index].link.link.clone()
                            />
                            <IconSelector
                                action=move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse() {
                                        set_settings.update(|s| s.repos[index].link.provider = value);
                                    }
                                }
                                prop=move || settings.read().repos[index].link.provider.to_string()
                            />
                            <Button
                                label="x"
                                action=move || {
                                    set_settings.update(|s| { s.repos.remove(index); });
                                }
                            />
                        </ScrollableBox>
                    </For>
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

/// Компонент для настройки секции "Социальные сети".
#[component]
pub fn SocialsSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();

    view! {
        <Section title="Social section">
            <Stack title="social sites">
                <ScrollXBar>
                <For
                    each= move || (0..settings.read().socials.len())
                    key= |index| *index
                    let:index
                >
                    <ScrollableBox>
                        <Input
                            action=move |ev| {
                                set_settings.update(|s| s.socials[index].link = ev.target().value());
                            }
                            prop=move || settings.read().socials[index].link.clone()
                        />
                        <IconSelector
                            action=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse() {
                                    set_settings.update(|s| s.socials[index].provider = value);
                                }
                            }
                            prop=move || settings.read().socials[index].provider.to_string()
                        />
                        <Button
                            label="x"
                            action=move || {
                                set_settings.update(|s| { s.socials.remove(index); });
                            }
                        />
                    </ScrollableBox>
                </For>
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

/// Компонент для настройки секции "Навыки".
#[component]
pub fn SkillsSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();
    let themes = use_context::<RW<Themes>>().unwrap().0;

    view! {
        <Section title="Skills section">
            <Stack title="skills">
                <ScrollXBar>
                    <For
                       each=move || (0..settings.read().skills.len())
                       key=|index| *index
                       let:index
                    >
                        <ScrollableBox>
                            <Input
                                action=move |ev| {
                                    set_settings.update(|s| s.skills[index].skill = ev.target().value());
                                }
                                prop=move || settings.read().skills[index].skill.clone()
                            />

                            <div class="grid grid-cols-2 gap-2">
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.skills[index].since.start = ev.target().value().parse().unwrap_or_default());
                                    }
                                    prop=move || settings.read().skills[index].since.start.to_string()
                                />
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.skills[index].since.end = ev.target().value().parse().unwrap_or_default());
                                    }
                                    prop=move || settings.read().skills[index].since.end.to_string()
                                />
                            </div>

                            <ScrollableBox>
                                <Input
                                    action=move |ev| {
                                        set_settings.update(|s| s.skills[index].repo_link.link = ev.target().value());
                                    }
                                    prop=move || settings.read().skills[index].repo_link.link.clone()
                                />
                                <IconSelector
                                    action=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse() {
                                            set_settings.update(|s| s.skills[index].repo_link.provider = value);
                                        }
                                    }
                                    prop=move || settings.read().skills[index].repo_link.provider.to_string()
                                />
                            </ScrollableBox>

                            <Stack title="projects">
                            <For
                               each=move || (0..settings.read().skills[index].projects.len())
                               key= |project_index| *project_index
                               let: project_index
                            >
                                <ScrollableBox>
                                    <Input
                                        action=move |ev| {
                                            set_settings.update(|s| s.skills[index].projects[project_index].name = ev.target().value());
                                        }
                                        prop=move || settings.read().skills[index].projects[project_index].name.clone()
                                    />
                                    <Input
                                        action=move |ev| {
                                            set_settings.update(|s| s.skills[index].projects[project_index].link.link = ev.target().value());
                                        }
                                        prop=move || settings.read().skills[index].projects[project_index].link.link.clone()
                                    />
                                    <IconSelector
                                        action=move |ev| {
                                            if let Ok(value) = event_target_value(&ev).parse() {
                                                set_settings.update(|s| s.skills[index].projects[project_index].link.provider = value);
                                            }
                                        }
                                        prop=move || settings.read().skills[index].projects[project_index].link.provider.to_string()
                                    />
                                    <Button
                                        label="x"
                                        action=move || {
                                            set_settings.update(|s| { s.skills[index].projects.remove(project_index); });
                                        }
                                    />
                                </ScrollableBox>
                              </For>
                                <Button
                                    label="+"
                                    action=move || {
                                        set_settings.update(|s| s.skills[index].projects.push(Project::default()));
                                    }
                                />
                            </Stack>

                            <button
                                on:click=move |_| {
                                    set_settings.update(|s| s.skills[index].main = !s.skills[index].main);
                                }
                                class="border"
                                style=move || {
                                    if !settings.read().skills[index].main {
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
                                    set_settings.update(|s| { s.skills.remove(index); });
                                }
                            />
                        </ScrollableBox>
                    </For>
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
