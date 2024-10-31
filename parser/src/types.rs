use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Имя
    pub name: String,
    /// Прозвище
    pub nickname: Nickname,
    /// Иные прозвища
    pub other_nicknames: Vec<Nickname>,
    /// Уклон в разработке
    pub specifications: Vec<String>,
    /// О пользователе
    pub about: About,
    /// Репозитории
    pub repos: Vec<Blank>,
    /// Социальные сети
    pub social_media: Vec<SocialMedia>,
    /// Список навыков
    pub skills: Vec<Skill>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nickname {
    /// Прозвище
    pub word: String,
    /// Произношение
    #[serde(default)]
    pub prononce: String, // опциональное поле
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub logo: String, // по умолчанию стандартный логотип
    /// Основной репозиторий
    #[serde(default)]
    pub main: bool, // по умолчанию false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialMedia {
    /// Провайдер соцсети
    pub provider: String,
    /// Ссылка на профиль
    pub link: String,
    /// Логотип соцсети
    #[serde(default)]
    pub logo: String, // по умолчанию стандартный логотип
    /// Основная соцсеть
    #[serde(default)]
    pub main: bool, // по умолчанию false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    /// Навык
    pub skill: String,
    /// Топ проектов
    pub top_projects: Vec<Blank>, // переиспользуем структуру Blank
    /// Даты
    #[serde(default)]
    pub since: Option<Since>, // опциональная структура
    /// Основной навык
    #[serde(default)]
    pub main: bool, // по умолчанию false
    /// Сайт поддержателя
    #[serde(default)]
    pub maintainer_site: String, // по умолчанию пустая строка
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Since {
    /// Год начала
    pub start: usize,
    /// Год окончания
    #[serde(default)]
    pub end: usize, // по умолчанию 0
}
