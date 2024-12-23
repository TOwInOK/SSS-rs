use serde::{Deserialize, Serialize};
use types::{render::Render, user::User};

pub mod types;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub user: User,
    pub render_type: Render,
}
