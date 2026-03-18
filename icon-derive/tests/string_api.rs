use std::str::FromStr;

mod common;

use common::Tabler;

#[test]
fn display_and_from_str_roundtrip_for_all_variants() {
    for icon in Tabler::all_icons() {
        let text = icon.to_string();
        assert_eq!(Tabler::from_str(&text).unwrap(), icon);
        assert_eq!(Tabler::from_str(&text.to_uppercase()).unwrap(), icon);
    }
}

#[test]
fn from_str_returns_error_for_unknown_variant() {
    let error = Tabler::from_str("outline_missing_icon").unwrap_err();
    assert!(error.contains("Unknown icon variant"));
}
