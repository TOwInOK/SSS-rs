use std::fmt::Debug;
pub mod prelude;
use prelude::{About, Skills, SocialMedias, Specifications, TopProjects};
use serde::{Deserialize, Serialize};
use types::{render::Render, user::User};
pub mod types;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub user: User,
    /// Уклон в разработке
    pub specifications: Specifications,
    /// О пользователе
    pub about: About,
    /// Репозитории
    pub repos: TopProjects,
    /// Социальные сети
    pub social_media: SocialMedias,
    /// Список навыков
    pub skills: Skills,
    /// Render type to render
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
