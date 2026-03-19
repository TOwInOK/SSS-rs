#![cfg(feature = "leptos")]

mod common;

use common::Tabler;
use leptos::prelude::RenderHtml;

#[test]
fn to_leptos_renders_embedded_svg_inside_wrapper_element() {
    let svg = Tabler::OUTLINE_SUPER_USER.as_str();
    let html = Tabler::OUTLINE_SUPER_USER.to_leptos().to_html();

    assert!(html.starts_with("<div"));
    assert!(html.contains(svg));
    assert!(html.contains("<svg"));
    assert!(html.contains("</svg>"));
    assert!(!html.contains("&lt;svg"));
}
