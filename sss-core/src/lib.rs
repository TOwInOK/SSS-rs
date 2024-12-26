use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use types::{render::Render, user::User};

pub mod types;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub user: User,
    pub render_type: Render,
}

impl UserProvider for Settings {
    fn user(&self) -> &User {
        &self.user
    }
}
impl RenderTypeProvider for Settings {
    fn render_type(&self) -> &Render {
        &self.render_type
    }
}

pub trait UserProvider
where
    Self: Debug + Default + Sized,
{
    fn user(&self) -> &User;
}

pub trait RenderTypeProvider
where
    Self: Debug + Default,
{
    fn render_type(&self) -> &Render;
}
