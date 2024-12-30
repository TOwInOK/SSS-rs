use components::prelude::*;
use sss_core::prelude::*;
use std::str::FromStr;

#[test]
fn test_icon_from_str() {
    // Test filled icons (default)
    assert_eq!(
        Icon::from_str("github").unwrap(),
        Icon::Filled(Filled::GitHub)
    );
    assert_eq!(Icon::from_str("gh").unwrap(), Icon::Filled(Filled::GitHub));
    assert_eq!(
        Icon::from_str("linkedin").unwrap(),
        Icon::Filled(Filled::LinkedIn)
    );
    assert_eq!(
        Icon::from_str("l.in").unwrap(),
        Icon::Filled(Filled::LinkedIn)
    );
    assert_eq!(
        Icon::from_str("telegram").unwrap(),
        Icon::Filled(Filled::Telegram)
    );
    assert_eq!(
        Icon::from_str("tg").unwrap(),
        Icon::Filled(Filled::Telegram)
    );

    // Test with explicit style
    assert_eq!(
        Icon::from_str("filled:github").unwrap(),
        Icon::Filled(Filled::GitHub)
    );
    assert_eq!(
        Icon::from_str("outline:github").unwrap(),
        Icon::Outline(Outline::GitHub)
    );

    // Test case insensitivity
    assert_eq!(
        Icon::from_str("FILLED:GITHUB").unwrap(),
        Icon::Filled(Filled::GitHub)
    );

    // Test custom icons
    assert_eq!(
        Icon::from_str("custom:icon").unwrap(),
        Icon::Custom("custom:icon".to_string())
    );
    assert_eq!(
        Icon::from_str("unknown").unwrap(),
        Icon::Custom("unknown".to_string())
    );
}

#[test]
fn test_icon_as_ref() {
    let github_filled = Icon::Filled(Filled::GitHub);
    let github_outline = Icon::Outline(Outline::GitHub);
    let custom = Icon::Custom("custom-svg".to_string());

    assert!(github_filled.as_ref().contains("width=\"24\""));
    assert!(github_outline.as_ref().contains("width=\"24\""));
    assert_eq!(custom.as_ref(), "custom-svg");
}

#[test]
fn test_filled_variants() {
    let github = Filled::GitHub;
    let linkedin = Filled::LinkedIn;
    let telegram = Filled::Telegram;

    assert!(github.as_str().contains("width=\"24\""));
    assert!(linkedin.as_str().contains("width=\"24\""));
    assert!(telegram.as_str().contains("width=\"24\""));
}

#[test]
fn test_outline_variants() {
    let github = Outline::GitHub;
    let linkedin = Outline::LinkedIn;
    let telegram = Outline::Telegram;

    assert!(github.as_str().contains("width=\"24\""));
    assert!(linkedin.as_str().contains("width=\"24\""));
    assert!(telegram.as_str().contains("width=\"24\""));
}

#[test]
fn test_sections_getters() {
    let mut sections = sections();

    // Setup initial data
    sections
        .specifications_mut()
        .extend(vec!["spec1".to_string(), "spec2".to_string()]);
    sections.socials_mut().push(Blank::default());
    sections.skills_mut().push(Skill::default());

    // Test getters
    let user = sections.user_info();
    assert_eq!(user.name, "");
    assert_eq!(user.nickname.word, "");
    assert_eq!(user.nickname.pronounce, None);
    assert!(user.other_nicknames.is_empty());

    assert_eq!(sections.specifications().len(), 2);
    assert_eq!(sections.specifications()[0], "spec1");
    assert_eq!(sections.about().text, "");
    assert_eq!(sections.socials().len(), 1);
    assert_eq!(sections.skills().len(), 1);
}

#[test]
fn test_sections_setters() {
    let mut sections = sections();

    // Test user_info setter
    let user = User::default()
        .name("John Doe".to_string())
        .nickname(
            Nickname::default()
                .word("johnny".to_string())
                .pronounce(Some("jon-ny".to_string())),
        )
        .other_nicknames(vec![Nickname::default()
            .word("joe".to_string())
            .pronounce(None)]);

    sections.set_user_info(user);
    assert_eq!(sections.user_info().name, "John Doe");
    assert_eq!(sections.user_info().nickname.word, "johnny");
    assert_eq!(
        sections.user_info().nickname.pronounce,
        Some("jon-ny".to_string())
    );
    assert_eq!(sections.user_info().other_nicknames[0].word, "joe");

    // Остальные тесты остаются без изменений
    let specs = vec!["spec3".to_string(), "spec4".to_string()];
    sections.set_specifications(specs);
    assert_eq!(sections.specifications().len(), 2);
    assert_eq!(sections.specifications()[1], "spec4");

    let mut about = About::default();
    about = about.text("About text".to_string());
    sections.set_about(about);
    assert_eq!(sections.about().text, "About text");

    let social = Blank::default().provider("GitHub".to_string());
    sections.set_socials(vec![social]);
    assert_eq!(sections.socials().len(), 1);
    assert_eq!(sections.socials()[0].provider, "GitHub");

    let skill = Skill::default().skill("Rust".to_string());
    sections.set_skills(vec![skill]);
    assert_eq!(sections.skills().len(), 1);
    assert_eq!(sections.skills()[0].skill, "Rust");
}

#[test]
fn test_sections_mut_accessors() {
    let mut sections = sections();

    // Test user_info_mut
    let nickname = Nickname::default()
        .word("jane".to_string())
        .pronounce(Some("jay-n".to_string()));

    sections.user_info_mut().name = "Jane Doe".to_string();
    sections.user_info_mut().nickname = nickname;

    assert_eq!(sections.user_info().name, "Jane Doe");
    assert_eq!(sections.user_info().nickname.word, "jane");
    assert_eq!(
        sections.user_info().nickname.pronounce,
        Some("jay-n".to_string())
    );

    // Остальные тесты остаются без изменений
    sections.specifications_mut().push("new spec".to_string());
    assert_eq!(sections.specifications().len(), 1);
    assert_eq!(sections.specifications()[0], "new spec");

    sections.about_mut().text = "New about text".to_string();
    assert_eq!(sections.about().text, "New about text");

    let social = Blank::default().provider("LinkedIn".to_string());
    sections.socials_mut().push(social);
    assert_eq!(sections.socials().len(), 1);
    assert_eq!(sections.socials()[0].provider, "LinkedIn");

    let skill = Skill::default().skill("Python".to_string());
    sections.skills_mut().push(skill);
    assert_eq!(sections.skills().len(), 1);
    assert_eq!(sections.skills()[0].skill, "Python");
}

#[test]
fn test_sections_default() {
    let sections = sections();

    assert_eq!(sections.user_info().name, "");
    assert_eq!(sections.user_info().nickname.word, "");
    assert_eq!(sections.user_info().nickname.pronounce, None);
    assert!(sections.user_info().other_nicknames.is_empty());
    assert!(sections.specifications().is_empty());
    assert_eq!(sections.about().text, "");
    assert!(sections.socials().is_empty());
    assert!(sections.skills().is_empty());
}
