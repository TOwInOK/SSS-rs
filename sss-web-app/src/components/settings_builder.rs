use crate::components::reusable_components::{
    button::ButtonStyle,
    input::{InputNumeric, InputUrl},
    prelude::*,
};
use leptos::prelude::*;
use render::layout::Limitations;
use sss_core::{
    Data,
    types::{
        link::Link,
        nickname::Nickname,
        skill::{Project, Skill},
    },
};
use sss_std::{prelude::HtmlLayouts, themes::Themes};

use crate::RW;

/// Main settings component, displays all settings sections.
#[component]
pub fn SettingsBuilder() -> impl IntoView {
    let limitations = use_context::<RW<HtmlLayouts>>().unwrap().0;

    view! {
        <div class="grid grid-cols-1 gap-4 h-full md:max-h-dvh md:overflow-y-scroll md:will-change-scroll">
            <Show
                when={move || limitations.get().is_user_section_allowed()}>
                <UserSection/>
            </Show>
            <Show
                when={move || limitations.get().is_about_section_allowed()}>
                <AboutSection/>
            </Show>
            <Show
                when={move || limitations.get().is_specification_section_allowed()}>
                <SpecificationsSection/>
            </Show>
            <Show
                when={move || limitations.get().is_repositories_section_allowed()}>
                <RepositoriesSection/>
            </Show>
            <Show
                when={move || limitations.get().is_socials_section_allowed()}>
                <SocialsSection/>
            </Show>
            <Show
                when={move || limitations.get().is_skills_section_allowed()}>
                <SkillsSection/>
            </Show>
        </div>
    }
}

/// Component for configuring user data.
#[component]
pub fn UserSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Data>>().unwrap();
    let limitations = use_context::<RW<HtmlLayouts>>().unwrap().0;

    view! {
        <SectionInverted title="User section">
            <Stack title="name">
                <Input
                    alt=|| "User name".to_string()
                    placeholder=|| "Enter your name (e.g. Dmitry)".to_string()
                    action=move |ev| {
                        set_settings.update(|x| x.layout.user.name = ev.target().value());
                    }
                    prop=move || {settings.get().layout.user.name}
                    maxlength=move || limitations.read().user_name_len()
                />
            </Stack>

            <Stack title="current name">
                <ScrollableBox>
                    <Input
                        alt=|| "Current nickname".to_string()
                        placeholder=|| "Enter nickname (e.g. TOwInOK)".to_string()
                        action=move |ev| {
                            set_settings.update(|x| x.layout.user.current_nickname.word = ev.target().value());
                        }
                        prop=move || settings.get().layout.user.current_nickname.word
                        maxlength=move || limitations.read().user_global_nickname_len()
                    />
                    <Input
                        alt=|| "Nickname pronunciation".to_string()
                        placeholder=|| "Enter pronunciation".to_string()
                        action=move |ev| {
                            set_settings.update(|x| x.layout.user.current_nickname.pronounce = ev.target().value());
                        }
                        prop=move || settings.get().layout.user.current_nickname.pronounce
                        maxlength=move || limitations.read().user_global_nickname_pronounce_len()
                    />
                </ScrollableBox>
            </Stack>

            <Show
                when={move || limitations.read().is_user_prevision_nicknames_section_allowed()}
                >
                <Stack title="prevision names">
                    <ScrollXBar>
                        <For
                            each=move || (0..settings.read().layout.user.prevision_nicknames.len())
                            key=|index| format!("prevision-names-stack-{}", index)
                            let:index
                        >
                            <ScrollableBox>
                                <Input
                                    alt=|| "Previous nickname".to_string()
                                    placeholder=|| "Enter previous nickname (e.g. nqcq)".to_string()
                                    action=move |ev| {
                                        set_settings.update(|x| x.layout.user.prevision_nicknames[index].word = ev.target().value());
                                    }
                                    prop=move || settings.read().layout.user.prevision_nicknames[index].word.clone()
                                    maxlength=move || limitations.read().user_global_nickname_len()
                                />
                                <Input
                                    alt=|| "Previous nickname pronunciation".to_string()
                                    placeholder=|| "Enter pronunciation".to_string()
                                    action=move |ev| {
                                        set_settings.update(|x| x.layout.user.prevision_nicknames[index].pronounce = ev.target().value());
                                    }
                                    prop=move || settings.read().layout.user.prevision_nicknames[index].pronounce.clone()
                                    maxlength=move || limitations.read().user_global_nickname_pronounce_len()
                                />
                                <Button
                                    alt=|| "Remove previous nickname".to_string()
                                    action=move || {
                                        set_settings.update(|x| {x.layout.user.prevision_nicknames.remove(index);});
                                    }
                                    style= ButtonStyle::Remove
                                />
                            </ScrollableBox>
                        </For>
                        <Show
                            when=move || settings.read().layout.specifications.len() < limitations.read().user_prevision_nicknames_max_count()
                        >
                        <Button
                            alt=|| "Add new previous nickname".to_string()
                            action=move || {
                                set_settings.update(|x| x.layout.user.prevision_nicknames.push(Nickname::default()));
                            }
                            style= ButtonStyle::Add
                        />
                        </Show>
                    </ScrollXBar>
                </Stack>
            </Show>

        </SectionInverted>
    }
}

/// Component for configuring the "About" section.
#[component]
pub fn AboutSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Data>>().unwrap();
    let limitations = use_context::<RW<HtmlLayouts>>().unwrap().0;

    view! {
        <SectionInverted title="About section">
            <Stack title="about">
                <TextArea
                    alt=|| "Text about yourself".to_string()
                    placeholder=|| "Write about yourself...".to_string()
                    action=move |ev| {
                        set_settings.update(|x| x.layout.about = ev.target().value());
                    }
                    prop=move || {settings.read().layout.about.clone()}
                    maxlength=move || limitations.read().about()
                />
            </Stack>
        </SectionInverted>
    }
}

#[component]
pub fn SpecificationsSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Data>>().unwrap();
    let limitations = use_context::<RW<HtmlLayouts>>().unwrap().0;
    view! {
        <SectionInverted title="Specifications section">
            <Stack title="specifications">
                <ScrollXBar>
                    <For
                        each=move || (0..settings.read().layout.specifications.len())
                        key=|index| format!("specifications-stack-{}", index)
                        let:index
                    >
                        <div class="grid gap-2 snap-start">
                            <Input
                                alt=|| "Specification".to_string()
                                placeholder=|| "Enter specification (e.g. Full-Stack developer)".to_string()
                                action=move |ev| {
                                    set_settings.update(|x| x.layout.specifications[index] = ev.target().value());
                                }
                                prop=move || settings.read().layout.specifications[index].clone()
                                maxlength=move || limitations.read().specifications_count().1
                            />
                            <Button
                                alt=|| "Remove specification".to_string()
                                action=move || {
                                    set_settings.update(|x| {x.layout.specifications.remove(index);});
                                }
                                style = ButtonStyle::Remove
                            />
                        </div>
                    </For>
                    <Show
                        when=move || settings.read().layout.specifications.len() < limitations.read().specifications_count().0
                    >
                        <Button
                            alt=|| "Add new specification".to_string()
                            action=move || {
                                set_settings.update(|x| x.layout.specifications.push(String::new()));
                            }
                            style = ButtonStyle::Add
                        />
                    </Show>
                </ScrollXBar>
            </Stack>
        </SectionInverted>
    }
}

/// Component for configuring the "Projects" section.
#[component]
pub fn RepositoriesSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Data>>().unwrap();
    let limitations = use_context::<RW<HtmlLayouts>>().unwrap().0;
    view! {
        <SectionInverted title="Projects section">
            <Stack title="repositories">
                <ScrollXBar>
                    <For
                        each=move || (0..settings.read().layout.repos.len())
                        key=|index| format!("projects-stack-{}", index)
                        let:index
                    >
                        <ScrollableBox>
                            <Input
                                alt=|| "Repository name".to_string()
                                placeholder=|| "Enter repository name (e.g. SSS-rs)".to_string()
                                action=move |ev| {
                                    set_settings.update(|x| x.layout.repos[index].name = ev.target().value());
                                }
                                prop=move || settings.read().layout.repos[index].name.clone()
                                maxlength=move || limitations.read().repositories_max_string_len()
                            />
                            <InputUrl
                                alt=|| "Repository link".to_string()
                                placeholder=|| "Enter repository URL".to_string()
                                action=move |ev| {
                                    set_settings.update(|x| x.layout.repos[index].link.link = ev.target().value());
                                }
                                prop=move || settings.read().layout.repos[index].link.link.clone()
                            />
                            <IconSelector
                                action=move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse() {
                                        set_settings.update(|x| x.layout.repos[index].link.icon = value);
                                    }
                                }
                                prop=move || settings.read().layout.repos[index].link.icon.to_string()
                            />

                            <Button
                                alt=|| "Remove repository".to_string()
                                action=move || {
                                    set_settings.update(|x| { x.layout.repos.remove(index); });
                                }
                                style = ButtonStyle::Remove
                            />
                        </ScrollableBox>
                    </For>
                    <Show
                        when=move || settings.read().layout.repos.len() < limitations.read().repositories_max_len()
                    >
                    <Button
                        alt=|| "Add new repository".to_string()
                        action=move || {
                            set_settings.update(|x| x.layout.repos.push(Project::default()));
                        }
                        style = ButtonStyle::Add
                    />
                    </Show>
                </ScrollXBar>
            </Stack>
        </SectionInverted>
    }
}

/// Component for configuring the "Social Networks" section.
#[component]
pub fn SocialsSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Data>>().unwrap();
    let limitations = use_context::<RW<HtmlLayouts>>().unwrap().0;
    view! {
        <SectionInverted title="Social section">
            <Stack title="social sites">
                <ScrollXBar>
                <For
                    each=move || (0..settings.read().layout.socials.len())
                        key=|index| format!("social-section-stack-{}", index)
                    let:index
                >
                    <ScrollableBox>
                        <InputUrl
                            alt=|| "Social link".to_string()
                            placeholder=|| "Enter social media URL".to_string()
                            action=move |ev| {
                                set_settings.update(|x| x.layout.socials[index].link = ev.target().value());
                            }
                            prop=move || settings.read().layout.socials[index].link.clone()
                        />
                        <IconSelector
                            action=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse() {
                                    set_settings.update(|x| x.layout.socials[index].icon = value);
                                }
                            }
                            prop=move || settings.read().layout.socials[index].icon.to_string()
                        />
                        <Button
                            alt=|| "Remove social link".to_string()
                            action=move || {
                                set_settings.update(|x| { x.layout.socials.remove(index); });
                            }
                            style = ButtonStyle::Remove
                        />
                    </ScrollableBox>
                </For>
                <Show
                    when=move || settings.read().layout.socials.len() < limitations.read().socials()
                >
                    <Button
                        alt=|| "Add new social link".to_string()
                        action=move || {
                            set_settings.update(|x| x.layout.socials.push(Link::default()));
                        }
                        style = ButtonStyle::Add
                    />
                </Show>
                </ScrollXBar>
            </Stack>
        </SectionInverted>
    }
}

/// Component for configuring the "Skills" section.
#[component]

pub fn SkillsSection() -> impl IntoView {
    let (settings, set_settings) = use_context::<RW<Data>>().unwrap();
    let themes = use_context::<RW<Themes>>().unwrap().0;
    let limitations = use_context::<RW<HtmlLayouts>>().unwrap().0;
    view! {
        <SectionInverted title="Skills section">
            <Stack title="skills">
                <ScrollXBar>
                    <For
                        each=move || 0..settings.read().layout.skills.len()
                            key=|index| format!("skills-section-stack-{}", index)
                        let:index
                    >
                        <ScrollableBox>
                            <Input
                                alt=|| "Skill name".to_string()
                                placeholder=|| "Enter skill name (e.g. Rust)".to_string()
                                action=move |ev| {
                                    set_settings.update(|x| x.layout.skills[index].skill = ev.target().value());
                                }
                                prop=move || settings.read().layout.skills[index].skill.clone()
                                maxlength=move || limitations.read().skills_skill_len()
                            />

                            <Show when=move || limitations.read().skills_since()>
                                <div class="grid grid-cols-2 gap-2">
                                    <InputNumeric
                                        alt=|| "Start year".to_string()
                                        placeholder=|| "Enter start year".to_string()
                                        action=move |ev| {
                                            set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.since.start = ev.target().value().parse::<usize>().unwrap_or_default()).unwrap_or_default());
                                        }
                                        prop=move || settings.with(|x| x.layout.skills.get(index).map(|x| x.since.start).unwrap_or_default()).to_string()
                                    />
                                    <InputNumeric
                                        alt=|| "End year".to_string()
                                        placeholder=|| "Enter end year".to_string()
                                        action=move |ev| {
                                            set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.since.end = ev.target().value().parse().unwrap_or_default()).unwrap_or_default());
                                        }
                                        prop=move || settings.with(|x| x.layout.skills.get(index).map(|x| x.since.end).unwrap_or_default()).to_string()
                                    />
                                </div>
                            </Show>

                            <Show
                                when=move || limitations.read().skills_repo_link()
                            >
                                <ScrollableBox>
                                    <InputUrl
                                        alt=|| "Repository link".to_string()
                                        placeholder=|| "Enter repository URL".to_string()
                                        action=move |ev| {
                                            set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.repo_link.link = ev.target().value()).unwrap_or_default());
                                        }
                                        prop=move || settings.with(|x| x.layout.skills.get(index).map(|x| x.repo_link.link.clone()).unwrap_or_default())
                                    />
                                    <IconSelector
                                        action=move |ev| {
                                            if let Ok(value) = event_target_value(&ev).parse() {
                                                set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.repo_link.icon = value).unwrap_or_default());
                                            }
                                        }
                                        prop=move || settings.with(|x| x.layout.skills.get(index).map(|x| x.repo_link.icon.to_string()).unwrap_or_default())
                                    />
                                </ScrollableBox>
                            </Show>

                            <Show
                                when=move || limitations.read().is_projects_in_skills_allowed()
                            >
                                <Stack title="projects">
                                <For
                                    each=move || (0..settings.read().layout.skills.get(index).map(|x| x.projects.len()).unwrap_or_default())
                                    key=|project_index| format!("skills-section-stack-project-section-stack-{}", project_index)
                                    let:project_index
                                >
                                    <ScrollableBox>
                                        <Input
                                            alt=|| "Project name".to_string()
                                            placeholder=|| "Enter project name".to_string()
                                            action=move |ev| {
                                                set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.projects[project_index].name = ev.target().value()).unwrap_or_default());
                                            }
                                            prop=move || settings.read().layout.skills.get(index).map(|x| x.projects[project_index].name.clone()).unwrap_or_default()
                                            maxlength=move || limitations.with(|x| x.skills_projects().1)
                                        />
                                        <InputUrl
                                            alt=|| "Project link".to_string()
                                            placeholder=|| "Enter project URL".to_string()
                                            action=move |ev| {
                                                set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.projects[project_index].link.link = ev.target().value()).unwrap_or_default());
                                            }
                                            prop=move || settings.read().layout.skills.get(index).map(|x| x.projects[project_index].link.link.clone()).unwrap_or_default()
                                        />
                                        <IconSelector
                                            action=move |ev| {
                                                if let Ok(value) = event_target_value(&ev).parse() {
                                                    set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.projects[project_index].link.icon = value).unwrap_or_default());
                                                }
                                            }
                                            prop=move || settings.read().layout.skills[index].projects[project_index].link.icon.to_string()
                                        />
                                        <Button
                                            alt=|| "Remove project".to_string()
                                            action=move || {
                                                set_settings.update(|x| { x.layout.skills.get_mut(index).map(|x| x.projects.remove(project_index)).unwrap_or_default(); });
                                            }
                                            style = ButtonStyle::Remove
                                        />
                                    </ScrollableBox>
                                </For>
                                <Show when=move || settings.read().layout.skills.get(index).map(|x| x.projects.len()).unwrap_or_default() < limitations.read().skills_projects().0 >
                                    <Button
                                        alt=|| "Add new project".to_string()
                                        action=move || {
                                            set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.projects.push(Project::default())).unwrap_or_default());
                                        }
                                        style = ButtonStyle::Add
                                    />
                                </Show>
                            </Stack>
                            </Show>

                            <Show when=move || limitations.read().skills_main()>
                                <button
                                    title="main"
                                    on:click=move |_| {
                                        set_settings.update(|x| x.layout.skills.get_mut(index).map(|x| x.main = !x.main).unwrap_or_default());
                                    }
                                    class="border"
                                    style=move || {
                                        if !settings.read().layout.skills.get(index).map(|x| x.main).unwrap_or_default() {
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
                            </Show>

                            <Button
                                alt=|| "Remove skill".to_string()
                                action=move || {
                                    set_settings.update(|x| { x.layout.skills.remove(index); });
                                }
                                style = ButtonStyle::Remove
                            />
                        </ScrollableBox>
                    </For>
                    <Show
                        when=move || settings.read().layout.skills.len() < limitations.read().skills_max_len()
                    >
                        <Button
                            alt=|| "Add new skill".to_string()
                            action=move || {
                                set_settings.update(|x| {
                                    x.layout.skills.push(Skill::default());
                                    if let Some(e) = x.layout.skills.last_mut() {
                                        e.projects.push(Project::default());
                                    }
                                });
                            }
                            style = ButtonStyle::Add
                        />
                    </Show>
                </ScrollXBar>
            </Stack>
        </SectionInverted>
    }
}
