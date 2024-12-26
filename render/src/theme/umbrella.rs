use super::{Colors, Gaps, Paddings, Shade, Theme};

pub static UMBRELLA_LIGHT: Shade<Theme> = Shade::Light(Theme {
    color: Colors {
        primary: "#1a1a1a",   // Почти чёрный
        secondary: "#4a4a4a", // Тёмно-серый
        thirdly: "#7a7a7a",   // Серый
        border: "#e5e5e5",    // Светло-серый
    },
    padding: Paddings {
        button: 2,
        border: 2,
        frame: 4,
    },
    gap: Gaps { frame: 2 },
});

pub static UMBRELLA_DARK: Shade<Theme> = Shade::Dark(Theme {
    color: Colors {
        primary: "#ffffff",   // Белый
        secondary: "#1a1a1a", // Светло-серый
        thirdly: "#a3a3a3",   // Серый
        border: "#ffffff",    // Тёмно-серый
    },
    padding: Paddings {
        button: 2,
        border: 1,
        frame: 4,
    },
    gap: Gaps { frame: 2 },
});
