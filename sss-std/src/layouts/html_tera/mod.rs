pub mod builder;
pub mod final_builder;
pub mod html_meta;
pub mod tools;
use html_layouts_derive::generate_layouts;

generate_layouts!(
    "src/layouts/html_tera/templates",
    "src/layouts/html_tera/final_templates"
);
