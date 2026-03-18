mod common;

use common::Tabler;

#[test]
fn generated_variants_are_available() {
    let _ = Tabler::OUTLINE_SUPER_USER;
    let _ = Tabler::FILLED_SUPER_USER;
    let _ = Tabler::OUTLINE_SUPER_TELEGRAM;
    let _ = Tabler::OUTLINE_CRATES;
}

#[test]
fn default_variant_is_the_first_icon() {
    assert_eq!(Tabler::default(), Tabler::OUTLINE_SUPER_USER);
}

#[test]
fn all_icons_returns_all_generated_variants_in_order() {
    assert_eq!(
        Tabler::all_icons(),
        vec![
            Tabler::OUTLINE_SUPER_USER,
            Tabler::FILLED_SUPER_USER,
            Tabler::OUTLINE_SUPER_TELEGRAM,
            Tabler::OUTLINE_CRATES,
        ]
    );
}
