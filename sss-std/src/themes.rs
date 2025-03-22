use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use theme::{Colors, Theme};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Themes {
    #[default]
    UMBRELLA,
    ROSE_PINE,
    GROOVEBOX,
    DRACULA,
    CATPPUCCIN_MAUVE,
    CATPPUCCIN_FLAMINGO,
    CATPPUCCIN_ROSEWATER,
    CATPPUCCIN_PINK,
    CATPPUCCIN_RED,
    CATPPUCCIN_MAROON,
    CATPPUCCIN_PEACH,
    CATPPUCCIN_YELLOW,
    CATPPUCCIN_GREEN,
    CATPPUCCIN_TEAL,
    CATPPUCCIN_SKY,
    CATPPUCCIN_SAPPHIRE,
    CATPPUCCIN_BLUE,
    CATPPUCCIN_LAVENDER,
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
            Self::CATPPUCCIN_MAUVE,
            Self::CATPPUCCIN_FLAMINGO,
            Self::CATPPUCCIN_ROSEWATER,
            Self::CATPPUCCIN_PINK,
            Self::CATPPUCCIN_RED,
            Self::CATPPUCCIN_MAROON,
            Self::CATPPUCCIN_PEACH,
            Self::CATPPUCCIN_YELLOW,
            Self::CATPPUCCIN_GREEN,
            Self::CATPPUCCIN_TEAL,
            Self::CATPPUCCIN_SKY,
            Self::CATPPUCCIN_SAPPHIRE,
            Self::CATPPUCCIN_BLUE,
            Self::CATPPUCCIN_LAVENDER,
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
            Themes::CATPPUCCIN_MAUVE => write!(f, "catppuccin_mauve"),
            Themes::CATPPUCCIN_FLAMINGO => write!(f, "catppuccin_flamingo"),
            Themes::CATPPUCCIN_ROSEWATER => write!(f, "catppuccin_rosewater"),
            Themes::CATPPUCCIN_PINK => write!(f, "catppuccin_pink"),
            Themes::CATPPUCCIN_RED => write!(f, "catppuccin_red"),
            Themes::CATPPUCCIN_MAROON => write!(f, "catppuccin_maroon"),
            Themes::CATPPUCCIN_PEACH => write!(f, "catppuccin_peach"),
            Themes::CATPPUCCIN_YELLOW => write!(f, "catppuccin_yellow"),
            Themes::CATPPUCCIN_GREEN => write!(f, "catppuccin_green"),
            Themes::CATPPUCCIN_TEAL => write!(f, "catppuccin_teal"),
            Themes::CATPPUCCIN_SKY => write!(f, "catppuccin_sky"),
            Themes::CATPPUCCIN_SAPPHIRE => write!(f, "catppuccin_sapphire"),
            Themes::CATPPUCCIN_BLUE => write!(f, "catppuccin_blue"),
            Themes::CATPPUCCIN_LAVENDER => write!(f, "catppuccin_lavender"),
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
            Themes::CATPPUCCIN_MAUVE => &CATPPUCCIN_MAUVE,
            Themes::CATPPUCCIN_FLAMINGO => &CATPPUCCIN_FLAMINGO,
            Themes::CATPPUCCIN_ROSEWATER => &CATPPUCCIN_ROSEWATER,
            Themes::CATPPUCCIN_PINK => &CATPPUCCIN_PINK,
            Themes::CATPPUCCIN_RED => &CATPPUCCIN_RED,
            Themes::CATPPUCCIN_MAROON => &CATPPUCCIN_MAROON,
            Themes::CATPPUCCIN_PEACH => &CATPPUCCIN_PEACH,
            Themes::CATPPUCCIN_YELLOW => &CATPPUCCIN_YELLOW,
            Themes::CATPPUCCIN_GREEN => &CATPPUCCIN_GREEN,
            Themes::CATPPUCCIN_TEAL => &CATPPUCCIN_TEAL,
            Themes::CATPPUCCIN_SKY => &CATPPUCCIN_SKY,
            Themes::CATPPUCCIN_SAPPHIRE => &CATPPUCCIN_SAPPHIRE,
            Themes::CATPPUCCIN_BLUE => &CATPPUCCIN_BLUE,
            Themes::CATPPUCCIN_LAVENDER => &CATPPUCCIN_LAVENDER,
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
            Themes::CATPPUCCIN_MAUVE => &CATPPUCCIN_MAUVE,
            Themes::CATPPUCCIN_FLAMINGO => &CATPPUCCIN_FLAMINGO,
            Themes::CATPPUCCIN_ROSEWATER => &CATPPUCCIN_ROSEWATER,
            Themes::CATPPUCCIN_PINK => &CATPPUCCIN_PINK,
            Themes::CATPPUCCIN_RED => &CATPPUCCIN_RED,
            Themes::CATPPUCCIN_MAROON => &CATPPUCCIN_MAROON,
            Themes::CATPPUCCIN_PEACH => &CATPPUCCIN_PEACH,
            Themes::CATPPUCCIN_YELLOW => &CATPPUCCIN_YELLOW,
            Themes::CATPPUCCIN_GREEN => &CATPPUCCIN_GREEN,
            Themes::CATPPUCCIN_TEAL => &CATPPUCCIN_TEAL,
            Themes::CATPPUCCIN_SKY => &CATPPUCCIN_SKY,
            Themes::CATPPUCCIN_SAPPHIRE => &CATPPUCCIN_SAPPHIRE,
            Themes::CATPPUCCIN_BLUE => &CATPPUCCIN_BLUE,
            Themes::CATPPUCCIN_LAVENDER => &CATPPUCCIN_LAVENDER,
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
            "catppuccin_mauve" => Ok(Self::CATPPUCCIN_MAUVE),
            "catppuccin_flamingo" => Ok(Self::CATPPUCCIN_FLAMINGO),
            "catppuccin_rosewater" => Ok(Self::CATPPUCCIN_ROSEWATER),
            "catppuccin_pink" => Ok(Self::CATPPUCCIN_PINK),
            "catppuccin_red" => Ok(Self::CATPPUCCIN_RED),
            "catppuccin_maroon" => Ok(Self::CATPPUCCIN_MAROON),
            "catppuccin_peach" => Ok(Self::CATPPUCCIN_PEACH),
            "catppuccin_yellow" => Ok(Self::CATPPUCCIN_YELLOW),
            "catppuccin_green" => Ok(Self::CATPPUCCIN_GREEN),
            "catppuccin_teal" => Ok(Self::CATPPUCCIN_TEAL),
            "catppuccin_sky" => Ok(Self::CATPPUCCIN_SKY),
            "catppuccin_sapphire" => Ok(Self::CATPPUCCIN_SAPPHIRE),
            "catppuccin_blue" => Ok(Self::CATPPUCCIN_BLUE),
            "catppuccin_lavender" => Ok(Self::CATPPUCCIN_LAVENDER),
            _ => Err(format!("Uncorrect theme: {}", s)),
        }
    }
}

pub static UMBRELLA: Theme = Theme {
    colors: Colors {
        text: "#7f69b5",
        background: "#371b1b",
        accent: "#de8cc5",
        border: "#7640bd",
    },
    font: (
        "PT Mono",
        "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
    ),
};

pub static ROSE_PINE: Theme = Theme {
    colors: Colors {
        text: "#F7D5C4",
        background: "#2D3142",
        accent: "#C3BAC6",
        border: "#564F5E",
    },
    font: (
        "PT Mono",
        "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
    ),
};

pub static GROOVEBOX: Theme = Theme {
    colors: Colors {
        text: "#ebdbb2",
        background: "#282828",
        accent: "#fb4934",
        border: "#32302f",
    },
    font: (
        "PT Mono",
        "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
    ),
};

pub static DRACULA: Theme = Theme {
    colors: Colors {
        text: "#F8F8F2",
        background: "#282A36",
        accent: "#FF79C6",
        border: "#44475A",
    },
    font: (
        "PT Mono",
        "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
    ),
};

pub static CATPPUCCIN_MAUVE: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#c6a0f6",     // Mauve
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_FLAMINGO: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#f0c6c6",     // Flamingo
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_ROSEWATER: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#f4dbd6",     // Rosewater
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_PINK: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#f5bde6",     // Pink
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_RED: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#ed8796",     // Red
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_MAROON: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#ee99a0",     // Maroon
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_PEACH: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#f5a97f",     // Peach
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_YELLOW: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#eed49f",     // Yellow
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_GREEN: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#a6da95",     // Green
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_TEAL: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#8bd5ca",     // Teal
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_SKY: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#91d7e3",     // Sky
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_SAPPHIRE: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#7dc4e4",     // Sapphire
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_BLUE: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#8aadf4",     // Blue
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};

pub static CATPPUCCIN_LAVENDER: Theme = Theme {
    colors: Colors {
        text: "#cad3f5",       // Text
        background: "#24273a", // Base
        accent: "#b7bdf8",     // Lavender
        border: "#494d64",     // Surface1
    },
    font: (
        "Fira Code",
        "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap",
    ),
};
