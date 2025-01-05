use std::{borrow::Cow, fs};

use components::prelude::*;
use encre_css::Preflight;
use render::prelude::*;
use sss_core::prelude::*;
use sss_std::{gen_css::gen_css, prelude::*};

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
        .logo(Some(Icon::Outline(Outline::GitHub).as_ref().into()));
    socials.push(github);

    // LinkedIn
    let linkedin = blank()
        .provider("l.in".to_string())
        .link("https://linkedin.com/in/username".to_string())
        .logo(Some(Icon::Outline(Outline::LinkedIn).as_ref().into()));
    socials.push(linkedin);

    // Telegram
    let telegram = blank()
        .provider("TG".to_string())
        .link("https://t.me/username".to_string())
        .logo(Some(Icon::Outline(Outline::Telegram).as_ref().into()));
    socials.push(telegram);

    sections.set_socials(socials);

    // Skills
    let mut skills = vec![];

    // Rust skill
    let rust = skill()
        .skill("Rust".to_string())
        .main(true)
        .since(Some(since().start(2020)))
        .site_label(Some("crates.io".to_string()))
        .top_projects(vec![]);
    skills.push(rust);

    // JS/TS skill
    let js = skill()
        .skill("JS/TS".to_string())
        .since(Some(since().start(2020)))
        .site_label(Some("gh.io".to_string()))
        .site_link(Some("#".to_string()))
        .top_projects(vec![]);
    skills.push(js);

    sections.set_skills(skills);

    // // Рендерим layout
    // let ub = UmbrellaHtmlRender;
    // let rendered = &ub.render(sections, &UMBRELLA);

    // // Проверяем наличие ключевых элементов в результате
    // assert!(rendered.contains("TOwInOK"));
    // assert!(rendered.contains("Ту́винок"));
    // assert!(rendered.contains("backend"));
    // assert!(rendered.contains("frontend"));
    // assert!(rendered.contains("Разработчик на Rust"));
    // assert!(rendered.contains("gh"));
    // assert!(rendered.contains("l.in"));
    // assert!(rendered.contains("TG"));
    // assert!(rendered.contains("Rust"));
    // assert!(rendered.contains("JS/TS"));
    // assert!(rendered.contains("2020 -> today"));
    // assert!(rendered.contains("crates.io"));
    // assert!(rendered.contains("gh.io"));

    // // Проверяем наличие необходимых стилей и структуры
    // assert!(rendered.contains("tailwind.config"));
    // assert!(rendered.contains("PT Mono"));
    // assert!(rendered.contains("min-h-screen"));
    // assert!(rendered.contains("flex justify-center"));

    // // Можно также сохранить результат в файл для визуальной проверки
    // std::fs::write("test_output.html", rendered).unwrap();

    let ub = UmbrellaHtmlTeraRender;
    let html = ub.render(sections, &UMBRELLA).unwrap();
    let mut css_config = encre_css::Config::default();
    css_config.preflight = Preflight::Full {
        ring_color: None,
        border_color: None,
        placeholder_color: None,
        font_family_sans: None,
        font_family_mono: Some(Cow::Borrowed("PT Mono")),
    };
    let css = gen_css(Some(css_config), &html);
    fs::write("card.html", html).unwrap();
    fs::write("card.css", css).unwrap();
}
