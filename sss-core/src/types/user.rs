use serde::{Deserialize, Serialize};

pub type Skills = Vec<Skill>;
pub type Nicknames = Vec<Nickname>;
pub type Specifications = Vec<String>;
pub type SocialMedias = Vec<Blank>;
pub type TopProjects = Vec<Blank>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    /// Имя
    pub name: String,
    /// Прозвище
    pub nickname: Nickname,
    /// Иные прозвища
    pub other_nicknames: Nicknames,
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
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Nickname {
    /// Прозвище
    pub word: String,
    /// Произношение
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronounce: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct About {
    /// Текст о пользователе
    pub text: String,
    /// Максимальная длина текста
    #[serde(default = "default_max_length")]
    pub max_length: usize, // по умолчанию 100
}

fn default_max_length() -> usize {
    100
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blank {
    /// Провайдер репозитория
    pub provider: String,
    /// Ссылка на репозиторий
    pub link: String,
    /// Логотип репозитория
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Основной репозиторий
    #[serde(default)]
    pub main: bool, // по умолчанию false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    /// Навык
    pub skill: String,
    /// Топ проектов
    pub top_projects: TopProjects,
    /// Даты
    #[serde(default)]
    pub since: Option<Since>, // опциональная структура
    /// Основной навык
    #[serde(default)]
    pub main: bool, // по умолчанию false
    /// Сайт поддержателя
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainer_site: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Since {
    /// Год начала
    pub start: usize,
    /// Год окончания
    #[serde(default)]
    pub end: usize, // по умолчанию 0
}
