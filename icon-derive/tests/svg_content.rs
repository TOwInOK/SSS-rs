mod common;

use common::Tabler;

#[test]
fn merged_name_uses_custom_display_name() {
    assert_eq!(Tabler::OUTLINE_SUPER_USER.as_str_merget(), "super_user");
    assert_eq!(
        Tabler::OUTLINE_SUPER_TELEGRAM.as_str_merget(),
        "super_telegram"
    );
    assert_eq!(Tabler::OUTLINE_CRATES.as_str_merget(), "Crates");
}

#[test]
fn embedded_svg_is_available() {
    let svg = Tabler::OUTLINE_SUPER_USER.as_str();
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("</svg>"));
}
