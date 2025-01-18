use std::fmt::Display;

use render::prelude::{Colors, Theme};
use serde::{Deserialize, Serialize};

macro_rules! define_themes {
     ($(
        $theme_name:ident => {
            primary: $primary:literal,
            secondary: $secondary:literal,
            thirdly: $thirdly:literal,
            border: $border:literal
        }
    ),* $(,)?) => {
        #[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum)]
        #[allow(non_camel_case_types)] /// Provide all themes in sss-std
    pub enum Themes {
            #[default]
            $($theme_name,)*
        }

        impl Display for Themes {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Themes::$theme_name => write!(f, "{}", stringify!($theme_name).to_lowercase()),)*
                }
            }
        }

        impl From<Themes> for &'static Theme {
            fn from(value: Themes) -> &'static Theme {
                match value {
                    $(Themes::$theme_name => &$theme_name,)*
                }
            }
        }

        impl From<&Themes> for &'static Theme {
            fn from(value: &Themes) -> &'static Theme {
                match value {
                    $(Themes::$theme_name => &$theme_name,)*
                }
            }
        }

        $(
            pub static $theme_name: Theme = Theme {
                colors: Colors {
                    primary: $primary,
                    secondary: $secondary,
                    thirdly: $thirdly,
                    border: $border,
                },
                regular_font: ("PT Serif", "https://fonts.googleapis.com/css2?family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap"),
                mono_font: ("PT Mono", "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap"),
            };
        )*
    };
}

define_themes! {
    UMBRELLA => {
        primary: "#7f69b5",
        secondary: "#371b1b",
        thirdly: "#de8cc5",
        border: "#7640bd"
        // regular_font: ("PT Serif", "https://fonts.googleapis.com/css2?family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap"),
        // mono_font: ("PT Mono", "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap")
    },
    ROSE_PINE => {
        primary: "#F7D5C4",
        secondary: "#2D3142",
        thirdly: "#C3BAC6",
        border: "#564F5E"
    },
    GROOVEBOX => {
        primary: "#ebdbb2",
        secondary: "#282828",
        thirdly: "#fb4934",
        border: "#32302f"
    },
    DRACULA => {
        primary: "#F8F8F2",
        secondary: "#282A36",
        thirdly: "#FF79C6",
        border: "#44475A"
    },
    DRAC2ULA => {
        primary: "#F8F8F2",
        secondary: "#282A36",
        thirdly: "#FF79C6",
        border: "#44475A"
    },
}
