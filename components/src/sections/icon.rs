use std::str::FromStr;

#[macro_export]
/// Return svg with under variant via name
/// Variants:
/// - Outline
/// - Filled
/// - Custom (push &str)
macro_rules! create_icon {
    ($t:ident, $item:ident) => {
        Icon::$t($t::$item)
    };
    ($t:ident, $item:expr) => {
        Icon::$t($item)
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Icon {
    Filled(Filled),
    Outline(Outline),
    Custom(String),
}
impl Icon {
    pub fn to_svg(
        &self,
        class: &str,
    ) -> String {
        match self {
            Icon::Filled(data) => format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" class="{}" {}</svg>"#,
                class,
                data.as_str()
            ),
            Icon::Outline(data) => format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" class="{}" {}</svg>"#,
                class,
                data.as_str()
            ),
            Icon::Custom(data) => format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" class="{}" {}</svg>"#,
                class,
                data.as_str()
            ),
        }
    }
}

impl AsRef<str> for Icon {
    fn as_ref(&self) -> &str {
        match self {
            Icon::Filled(filled) => filled.as_str(),
            Icon::Outline(outline) => outline.as_str(),
            Icon::Custom(e) => e.as_ref(),
        }
    }
}
impl FromStr for Icon {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_lowercase();
        let parts: Vec<&str> = binding.split(':').collect();
        match parts.as_slice() {
            // Если указан тип (filled/outline)
            [style, name] => match *style {
                "filled" => match *name {
                    "github" | "gh" => Ok(Icon::Filled(Filled::GitHub)),
                    "linkedin" | "l.in" => Ok(Icon::Filled(Filled::LinkedIn)),
                    "telegram" | "tg" => Ok(Icon::Filled(Filled::Telegram)),
                    _ => Ok(Icon::Custom(s.to_string())),
                },
                "outline" => match *name {
                    "github" | "gh" => Ok(Icon::Outline(Outline::GitHub)),
                    "linkedin" | "l.in" => Ok(Icon::Outline(Outline::LinkedIn)),
                    "telegram" | "tg" => Ok(Icon::Outline(Outline::Telegram)),
                    _ => Ok(Icon::Custom(s.to_string())),
                },
                _ => Ok(Icon::Custom(s.to_string())),
            },
            // Если тип не указан, используем Filled по умолчанию
            [name] => match *name {
                "github" | "gh" => Ok(Icon::Filled(Filled::GitHub)),
                "linkedin" | "l.in" => Ok(Icon::Filled(Filled::LinkedIn)),
                "telegram" | "tg" => Ok(Icon::Filled(Filled::Telegram)),
                _ => Ok(Icon::Custom(s.to_string())),
            },
            // Если формат неверный, возвращаем Custom
            _ => Ok(Icon::Custom(s.to_string())),
        }
    }
}
/// Gen outline svg
macro_rules! icon_outline {
    ($($name:ident($svg:expr)),+ $(,)?) => {

        #[derive(Debug, Clone, PartialEq)]
        pub enum Outline {
            $($name),*
        }

        impl Outline {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        Outline::$name => $svg
                    ),*
                }
            }
        }
    };
}
/// Gen filled svg
macro_rules! icon_filled {
    ($($name:ident($svg:expr)),+ $(,)?) => {

        #[derive(Debug, Clone, PartialEq)]
        pub enum Filled {
            $($name),*
        }
        impl Filled {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        Filled::$name => $svg
                     ),*
                }
            }
        }
    };
}

icon_filled!(
    GitHub(
        r#"width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.""#
    ),
    LinkedIn(
        r#"width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M8 11v5" /><path d="M8 8v.01" /><path d="M12 16v-5" /><path d="M16 16v-3a2 2 0 1 0 -4 0" /><path d="M3 7a4 4 0 0 1 4 -4h10a4 4 0 0 1 4 4v10a4 4 0 0 1 -4 4h-10a4 4 0 0 1 -4 -4z""#
    ),
    Telegram(
        r#"width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M15 10l-4 4l6 6l4 -16l-18 7l4 2l2 6l3 -4""#
    ),
);

icon_outline!(
    GitHub(
        r#"width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.""#
    ),
    LinkedIn(
        r#"width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M8 11v5" /><path d="M8 8v.01" /><path d="M12 16v-5" /><path d="M16 16v-3a2 2 0 1 0 -4 0" /><path d="M3 7a4 4 0 0 1 4 -4h10a4 4 0 0 1 4 4v10a4 4 0 0 1 -4 4h-10a4 4 0 0 1 -4 -4z""#
    ),
    Telegram(
        r#"width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M15 10l-4 4l6 6l4 -16l-18 7l4 2l2 6l3 -4""#
    ),
);
