use components::prelude::*;
use render::prelude::*;
use sss_core::prelude::*;
use sss_std::{layouts::umbrella_html::UmbrellaHtmlRender, theme::umbrella::UMBRELLA};

#[test]
fn test_umbrella_layout() {
    // Создаем тестовые данные
    let mut sections = sections();

    // User Info
    let user = user()
        .name("Test User".to_string())
        .nickname(
            nickname()
                .word("TOwInOK".to_string())
                .pronounce(Some("Ту́винок".to_string())),
        )
        .other_nicknames(vec![]);
    sections.set_user_info(user);

    // Specifications
    sections.set_specifications(vec!["backend".to_string(), "frontend".to_string()]);

    // About
    let about = about()
        .text("Разработчик на Rust, который любит делать хорошо".to_string())
        .max_length(100);
    sections.set_about(about);

    // Social Media
    let mut socials = vec![];

    // GitHub
    let github = blank()
        .provider("gh".to_string())
        .link("https://github.com/username".to_string())
        .logo(Some(r#"viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
            <path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5"
            />"#.to_string()));
    socials.push(github);

    // LinkedIn
    let linkedin = blank()
        .provider("l.in".to_string())
        .link("https://linkedin.com/in/username".to_string())
        .logo(Some(r#"viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
            <path d="M8 11v5"/><path d="M8 8v.01"/>
            <path d="M12 16v-5"/>
            <path d="M16 16v-3a2 2 0 1 0 -4 0"/>
            <path d="M3 7a4 4 0 0 1 4 -4h10a4 4 0 0 1 4 4v10a4 4 0 0 1 -4 4h-10a4 4 0 0 1 -4 -4z"/>
            "#.to_string()));
    socials.push(linkedin);

    // Telegram
    let telegram = blank()
        .provider("TG".to_string())
        .link("https://t.me/username".to_string())
        .logo(Some(r#"viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
            <path d="M15 10l-4 4l6 6l4 -16l-18 7l4 2l2 6l3 -4"/>
            "#.to_string()));
    socials.push(telegram);

    sections.set_socials(socials);

    // Skills
    let mut skills = vec![];

    // Rust skill
    let rust = skill()
        .skill("Rust".to_string())
        .main(true)
        .since(Some(since().start(2020)))
        .maintainer_site(Some("crates.io".to_string()))
        .top_projects(vec![]);
    skills.push(rust);

    // JS/TS skill
    let js = skill()
        .skill("JS/TS".to_string())
        .since(Some(since().start(2020)))
        .maintainer_site(Some("gh.io".to_string()))
        .top_projects(vec![]);
    skills.push(js);

    sections.set_skills(skills);

    // Рендерим layout
    let rendered = UmbrellaHtmlRender::render(sections, &UMBRELLA);
    let final_html = UmbrellaHtmlRender::finylize(rendered, &UMBRELLA);

    // Проверяем наличие ключевых элементов в результате
    assert!(final_html.contains("TOwInOK"));
    assert!(final_html.contains("Ту́винок"));
    assert!(final_html.contains("backend"));
    assert!(final_html.contains("frontend"));
    assert!(final_html.contains("Разработчик на Rust"));
    assert!(final_html.contains("gh"));
    assert!(final_html.contains("l.in"));
    assert!(final_html.contains("TG"));
    assert!(final_html.contains("Rust"));
    assert!(final_html.contains("JS/TS"));
    assert!(final_html.contains("2020 -> today"));
    assert!(final_html.contains("crates.io"));
    assert!(final_html.contains("gh.io"));

    // Проверяем наличие необходимых стилей и структуры
    assert!(final_html.contains("tailwind.config"));
    assert!(final_html.contains("PT Mono"));
    assert!(final_html.contains("min-h-screen"));
    assert!(final_html.contains("flex justify-center"));

    // Можно также сохранить результат в файл для визуальной проверки
    std::fs::write("test_output.html", final_html).unwrap();
}
