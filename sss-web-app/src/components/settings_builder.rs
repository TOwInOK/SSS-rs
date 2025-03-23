use crate::components::reusable_components::{button::AddButton, prelude::*};
use leptos::prelude::*;
use sss_core::{
    Settings,
    types::{
        link::Link,
        nickname::Nickname,
        skill::{Project, Skill},
    },
};
use sss_std::themes::Themes;

use crate::RW;

/// Main settings component, displays all settings sections.
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

/// Component for configuring user data.
#[component]
pub fn UserSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();
    view! {
        <Section title="User section">
            <Stack title="name">
                <Input
                    alt=|| "User name".to_string()
                    placeholder=|| "Enter your name (e.g. Dmitry)".to_string()
                    action=move |ev| {
                        set_settings.update(|x| x.user.name = ev.target().value());
                    }
                    prop=move || {settings.get().user.name}
                />
            </Stack>

            <Stack title="current name">
                <ScrollableBox>
                    <Input
                        alt=|| "Current nickname".to_string()
                        placeholder=|| "Enter nickname (e.g. TOwInOK)".to_string()
                        action=move |ev| {
                            set_settings.update(|x| x.user.current_nickname.word = ev.target().value());
                        }
                        prop=move || settings.get().user.current_nickname.word
                    />
                    <Input
                        alt=|| "Nickname pronunciation".to_string()
                        placeholder=|| "Enter pronunciation".to_string()
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
                        each=move || (0..settings.read().user.prevision_nicknames.len())
                        key=|index| format!("prevision-names-stack-{}", index)
                        let:index
                    >
                        <ScrollableBox>
                            <Input
                                alt=|| "Previous nickname".to_string()
                                placeholder=|| "Enter previous nickname (e.g. nqcq)".to_string()
                                action=move |ev| {
                                    set_settings.update(|s| s.user.prevision_nicknames[index].word = ev.target().value());
                                }
                                prop=move || settings.read().user.prevision_nicknames[index].word.clone()
                            />
                            <Input
                                alt=|| "Previous nickname pronunciation".to_string()
                                placeholder=|| "Enter pronunciation".to_string()
                                action=move |ev| {
                                    set_settings.update(|s| s.user.prevision_nicknames[index].pronounce = ev.target().value());
                                }
                                prop=move || settings.read().user.prevision_nicknames[index].pronounce.clone()
                            />
                            <Button
                                alt=|| "Remove previous nickname".to_string()
                                label="x"
                                action=move || {
                                    set_settings.update(|s| {s.user.prevision_nicknames.remove(index);});
                                }
                            />
                        </ScrollableBox>
                    </For>
                    <AddButton
                        alt=|| "Add new previous nickname".to_string()
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

/// Component for configuring the "About" section.
#[component]
pub fn AboutSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();

    view! {
        <Section title="About section">
            <Stack title="about">
                <TextArea
                    alt=|| "Text about yourself".to_string()
                    placeholder=|| "Write about yourself...".to_string()
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
                        key=|index| format!("specifications-stack-{}", index)
                        let:index
                    >
                        <div class="grid gap-2 snap-start">
                            <Input
                                alt=|| "Specification".to_string()
                                placeholder=|| "Enter specification (e.g. Full-Stack developer)".to_string()
                                action=move |ev| {
                                    set_settings.update(|x| x.specifications[index] = ev.target().value());
                                }
                                prop=move || settings.read().specifications[index].clone()
                            />
                            <Button
                                alt=|| "Remove specification".to_string()
                                label="x"
                                action=move || {
                                    set_settings.update(|x| {x.specifications.remove(index);});
                                }
                            />
                        </div>
                    </For>
                    <AddButton
                        alt=|| "Add new specification".to_string()
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

/// Component for configuring the "Projects" section.
#[component]
pub fn RepositoriesSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();

    view! {
        <Section title="Projects section">
            <Stack title="repositories">
                <ScrollXBar>
                    <For
                        each=move || (0..settings.read().repos.len())
                        key=|index| format!("projects-stack-{}", index)
                        let:index
                    >
                        <ScrollableBox>
                            <Input
                                alt=|| "Repository name".to_string()
                                placeholder=|| "Enter repository name (e.g. SSS-rs)".to_string()
                                action=move |ev| {
                                    set_settings.update(|s| s.repos[index].name = ev.target().value());
                                }
                                prop=move || settings.read().repos[index].name.clone()
                            />
                            <Input
                                alt=|| "Repository link".to_string()
                                placeholder=|| "Enter repository URL".to_string()
                                action=move |ev| {
                                    set_settings.update(|s| s.repos[index].link.link = ev.target().value());
                                }
                                prop=move || settings.read().repos[index].link.link.clone()
                            />
                            <IconSelector
                                action=move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse() {
                                        set_settings.update(|s| s.repos[index].link.icon = value);
                                    }
                                }
                                prop=move || settings.read().repos[index].link.icon.to_string()
                            />
                            <Button
                                alt=|| "Remove repository".to_string()
                                label="x"
                                action=move || {
                                    set_settings.update(|s| { s.repos.remove(index); });
                                }
                            />
                        </ScrollableBox>
                    </For>
                    <AddButton
                        alt=|| "Add new repository".to_string()
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

/// Component for configuring the "Social Networks" section.
#[component]
pub fn SocialsSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Settings>>().unwrap();

    view! {
        <Section title="Social section">
            <Stack title="social sites">
                <ScrollXBar>
                <For
                    each=move || (0..settings.read().socials.len())
                        key=|index| format!("social-section-stack-{}", index)
                    let:index
                >
                    <ScrollableBox>
                        <Input
                            alt=|| "Social link".to_string()
                            placeholder=|| "Enter social media URL".to_string()
                            action=move |ev| {
                                set_settings.update(|s| s.socials[index].link = ev.target().value());
                            }
                            prop=move || settings.read().socials[index].link.clone()
                        />
                        <IconSelector
                            action=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse() {
                                    set_settings.update(|s| s.socials[index].icon = value);
                                }
                            }
                            prop=move || settings.read().socials[index].icon.to_string()
                        />
                        <Button
                            alt=|| "Remove social link".to_string()
                            label="x"
                            action=move || {
                                set_settings.update(|s| { s.socials.remove(index); });
                            }
                        />
                    </ScrollableBox>
                </For>
                    <AddButton
                        alt=|| "Add new social link".to_string()
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

/// Component for configuring the "Skills" section.
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
                            key=|index| format!("skills-section-stack-{}", index)
                        let:index
                    >
                        <ScrollableBox>
                            <Input
                                alt=|| "Skill name".to_string()
                                placeholder=|| "Enter skill name (e.g. Rust)".to_string()
                                action=move |ev| {
                                    set_settings.update(|s| s.skills[index].skill = ev.target().value());
                                }
                                prop=move || settings.read().skills[index].skill.clone()
                            />

                            <div class="grid grid-cols-2 gap-2">
                                <Input
                                    alt=|| "Start year".to_string()
                                    placeholder=|| "Enter start year".to_string()
                                    action=move |ev| {
                                        set_settings.update(|s| s.skills[index].since.start = ev.target().value().parse().unwrap_or_default());
                                    }
                                    prop=move || settings.read().skills[index].since.start.to_string()
                                />
                                <Input
                                    alt=|| "End year".to_string()
                                    placeholder=|| "Enter end year".to_string()
                                    action=move |ev| {
                                        set_settings.update(|s| s.skills[index].since.end = ev.target().value().parse().unwrap_or_default());
                                    }
                                    prop=move || settings.read().skills[index].since.end.to_string()
                                />
                            </div>

                            <ScrollableBox>
                                <Input
                                    alt=|| "Repository link".to_string()
                                    placeholder=|| "Enter repository URL".to_string()
                                    action=move |ev| {
                                        set_settings.update(|s| s.skills[index].repo_link.link = ev.target().value());
                                    }
                                    prop=move || settings.read().skills[index].repo_link.link.clone()
                                />
                                <IconSelector
                                    action=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse() {
                                            set_settings.update(|s| s.skills[index].repo_link.icon = value);
                                        }
                                    }
                                    prop=move || settings.read().skills[index].repo_link.icon.to_string()
                                />
                            </ScrollableBox>

                            <Stack title="projects">
                                <For
                                    each=move || (0..settings.read().skills[index].projects.len())
                                    key=|project_index| format!("skills-section-stack-project-section-stack-{}", project_index)
                                    let:project_index
                                >
                                    <ScrollableBox>
                                        <Input
                                            alt=|| "Project name".to_string()
                                            placeholder=|| "Enter project name".to_string()
                                            action=move |ev| {
                                                set_settings.update(|s| s.skills[index].projects[project_index].name = ev.target().value());
                                            }
                                            prop=move || settings.read().skills[index].projects[project_index].name.clone()
                                        />
                                        <Input
                                            alt=|| "Project link".to_string()
                                            placeholder=|| "Enter project URL".to_string()
                                            action=move |ev| {
                                                set_settings.update(|s| s.skills[index].projects[project_index].link.link = ev.target().value());
                                            }
                                            prop=move || settings.read().skills[index].projects[project_index].link.link.clone()
                                        />
                                        <IconSelector
                                            action=move |ev| {
                                                if let Ok(value) = event_target_value(&ev).parse() {
                                                    set_settings.update(|s| s.skills[index].projects[project_index].link.icon = value);
                                                }
                                            }
                                            prop=move || settings.read().skills[index].projects[project_index].link.icon.to_string()
                                        />
                                        <Button
                                            alt=|| "Remove project".to_string()
                                            label="x"
                                            action=move || {
                                                set_settings.update(|s| { s.skills[index].projects.remove(project_index); });
                                            }
                                        />
                                    </ScrollableBox>
                                </For>
                                <AddButton
                                    alt=|| "Add new project".to_string()
                                    label="+"
                                    action=move || {
                                        set_settings.update(|s| s.skills[index].projects.push(Project::default()));
                                    }
                                />
                            </Stack>

                            <button
                                title="Change main state"
                                on:click=move |_| {
                                    set_settings.update(|s| s.skills[index].main = !s.skills[index].main);
                                }
                                class="border"
                                style=move || {
                                    if !settings.read().skills[index].main {
                                        format!(
                                            "background-color: {}; color: {}; border-color: {};",
                                            themes.get().colors().background,
                                            themes.get().colors().text,
                                            themes.get().colors().text
                                        )
                                    } else {
                                        format!(
                                            "background-color: {}; color: {}; border-color: {};",
                                            themes.get().colors().text,
                                            themes.get().colors().background,
                                            themes.get().colors().background
                                        )
                                    }
                                }
                            >
                                main
                            </button>

                            <Button
                                alt=|| "Remove skill".to_string()
                                label="x"
                                action=move || {
                                    set_settings.update(|s| { s.skills.remove(index); });
                                }
                            />
                        </ScrollableBox>
                    </For>
                    <AddButton
                        alt=|| "Add new skill".to_string()
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
