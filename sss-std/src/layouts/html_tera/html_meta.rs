use serde::{Deserialize, Serialize};
use sss_core::Settings;
use tera::Context;
use theme::Theme;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Meta {
    /// Title of the page
    pub title: String,
    /// description & about
    pub description: String,
    /// User name
    /// Current nickname
    pub author: String,
    /// Keywords for the page
    pub keywords: Vec<String>,
    // Image settings
    pub image_link: String,
    pub image_link_secure: String,
    pub image_type: String,
    /// OG specific
    pub og_locale: String,
    pub og_site_name: String,
    pub og_type: String,
    // IDK
    // pub og_url: String,
    pub discord_theme_color: String,
}

impl Meta {
    pub fn insert_to_context(
        &self,
        context: &mut Context,
    ) {
        context.insert("meta_title", &self.title);
        context.insert("meta_description", &self.description);
        context.insert("meta_author", &self.author);
        context.insert("meta_keywords_list", &self.keywords);
        context.insert("meta_keywords_string", &self.keywords.join(", "));
        context.insert("meta_image_link", &self.image_link);
        context.insert("meta_image_link_secure", &self.image_link_secure);
        context.insert("meta_image_type", &self.image_type);
        context.insert("meta_og_locale", &self.og_locale);
        context.insert("meta_og_site_name", &self.og_site_name);
        context.insert("meta_og_type", &self.og_type);
        context.insert("meta_discord_theme_color", &self.discord_theme_color);
    }
}

impl From<(&Settings, &Theme)> for Meta {
    fn from(value: (&Settings, &Theme)) -> Self {
        Self {
            title: format!("{}'s card", value.0.user.current_nickname.word),
            description: value.0.about.clone(),
            author: value.0.user.current_nickname.word.clone(),
            keywords: {
                let mut c = value.0.specifications.clone();
                c.append(&mut value.0.skills.iter().cloned().map(|x| x.skill).collect());
                c.push(value.0.user.current_nickname.word.clone());
                c.push("Card".to_string());
                c.push("SSS-rs".to_string());
                c.push("Portfolio".to_string());
                c
            },
            image_link:
                r#"https://github.com/TOwInOK/SSS-rs/blob/main/.content/preview.webp?raw=true"#
                    .to_string(),
            image_link_secure:
                r#"https://github.com/TOwInOK/SSS-rs/blob/main/.content/preview.webp?raw=true"#
                    .to_string(),
            image_type: r#"image/webp"#.to_string(),
            og_locale: "en_US".to_string(),
            og_site_name: format!("{}'s card", value.0.user.current_nickname.word),
            og_type: "website".to_string(),
            discord_theme_color: value.1.colors.accent.to_string(),
        }
    }
}
