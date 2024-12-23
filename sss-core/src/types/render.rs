use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum Render {
    #[default]
    HTML,
    LEPTOS,
    HTMLCSS,
}
