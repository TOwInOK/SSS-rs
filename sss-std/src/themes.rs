use std::fmt::Display;

use render::prelude::{Colors, Theme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum)]
pub enum Themes {
    #[default]
    Umbrella,
    RosePine,
    GrooveBox,
    Dracula,
}

impl Display for Themes {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Themes::Umbrella => write!(f, "umbrella"),
            Themes::RosePine => write!(f, "rose-pine"),
            Themes::GrooveBox => write!(f, "groovebox"),
            Themes::Dracula => write!(f, "dracula"),
        }
    }
}
impl From<Themes> for &'static Theme {
    fn from(value: Themes) -> &'static Theme {
        match value {
            Themes::Umbrella => &UMBRELLA,
            Themes::RosePine => &ROSE_PINE,
            Themes::GrooveBox => &GROOVEBOX,
            Themes::Dracula => &DRACULA,
        }
    }
}

pub static UMBRELLA: Theme = Theme {
    colors: Colors {
        primary: "#7f69b5",   // Primary text color (light purple)
        secondary: "#371b1b", // Background color (dark brown)
        thirdly: "#de8cc5",   // Accent color (pink)
        border: "#7640bd",    // Color for secondary elements (dark purple)
    },
};

pub static ROSE_PINE: Theme = Theme {
    colors: Colors {
        primary: "#F7D5C4",   // Primary text color (rosy)
        secondary: "#2D3142", // Background color (dark blue)
        thirdly: "#C3BAC6",   // Accent color (rosy-lilac)
        border: "#564F5E",    // Color for secondary elements (dark purple)
    },
};

pub static GROOVEBOX: Theme = Theme {
    colors: Colors {
        primary: "#ebdbb2",   // Primary text color
        secondary: "#282828", // Background color
        thirdly: "#fb4934",   // Accent color
        border: "#32302f",    // Color for secondary elements
    },
};

pub static DRACULA: Theme = Theme {
    colors: Colors {
        primary: "#F8F8F2",   // Primary text color (white)
        secondary: "#282A36", // Background color (dark gray)
        thirdly: "#FF79C6",   // Accent color (pink)
        border: "#44475A",    // Color for secondary elements (dark gray)
    },
};
