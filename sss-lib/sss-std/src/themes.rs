use render::prelude::{Colors, Theme};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Themes {
    #[default]
    UMBRELLA,
    ROSE_PINE,
    GROOVEBOX,
    DRACULA,
}

impl Themes {
    pub fn font(&self) -> (&'static str, &'static str) {
        let theme: &Theme = self.into();
        theme.font
    }
    pub fn colors(&self) -> &Colors {
        let theme: &Theme = self.into();
        &theme.colors
    }
}

impl Themes {
    // Methot to return all avaiable [Themes]
    pub fn all_themes() -> Vec<Self> {
        vec![
            Self::UMBRELLA,
            Self::ROSE_PINE,
            Self::DRACULA,
            Self::GROOVEBOX,
        ]
    }
}

impl Display for Themes {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Themes::UMBRELLA => write!(f, "umbrella"),
            Themes::ROSE_PINE => write!(f, "rose_pine"),
            Themes::GROOVEBOX => write!(f, "groovebox"),
            Themes::DRACULA => write!(f, "dracula"),
        }
    }
}

impl From<Themes> for &'static Theme {
    fn from(value: Themes) -> &'static Theme {
        match value {
            Themes::UMBRELLA => &UMBRELLA,
            Themes::ROSE_PINE => &ROSE_PINE,
            Themes::GROOVEBOX => &GROOVEBOX,
            Themes::DRACULA => &DRACULA,
        }
    }
}

impl From<&Themes> for &'static Theme {
    fn from(value: &Themes) -> &'static Theme {
        match value {
            Themes::UMBRELLA => &UMBRELLA,
            Themes::ROSE_PINE => &ROSE_PINE,
            Themes::GROOVEBOX => &GROOVEBOX,
            Themes::DRACULA => &DRACULA,
        }
    }
}

impl FromStr for Themes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.as_str();
        match s {
            "umbrella" => Ok(Self::UMBRELLA),
            "rose_pine" => Ok(Self::ROSE_PINE),
            "groovebox" => Ok(Self::GROOVEBOX),
            "dracula" => Ok(Self::DRACULA),
            _ => Err(format!("Uncorrect theme: {}", s)),
        }
    }
}

pub static UMBRELLA: Theme = Theme {
    colors: Colors {
        primary: "#7f69b5",
        secondary: "#371b1b",
        thirdly: "#de8cc5",
        border: "#7640bd",
    },
    font: (
        "PT Mono",
        "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
    ),
};

pub static ROSE_PINE: Theme = Theme {
    colors: Colors {
        primary: "#F7D5C4",
        secondary: "#2D3142",
        thirdly: "#C3BAC6",
        border: "#564F5E",
    },
    font: (
        "PT Mono",
        "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
    ),
};

pub static GROOVEBOX: Theme = Theme {
    colors: Colors {
        primary: "#ebdbb2",
        secondary: "#282828",
        thirdly: "#fb4934",
        border: "#32302f",
    },
    font: (
        "PT Mono",
        "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
    ),
};

pub static DRACULA: Theme = Theme {
    colors: Colors {
        primary: "#F8F8F2",
        secondary: "#282A36",
        thirdly: "#FF79C6",
        border: "#44475A",
    },
    font: (
        "PT Mono",
        "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
    ),
};
