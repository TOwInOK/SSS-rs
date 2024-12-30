use super::{Colors, Gaps, Paddings, Shade, Theme};

pub static UMBRELLA_LIGHT: Shade<Theme> = Shade::Light(Theme {
    color: Colors {
        primary: "#7f69b5",   // Основной цвет текста (светло-фиолетовый)
        secondary: "#371b1b", // Цвет фона (тёмно-коричневый)
        thirdly: "#de8cc5",   // Акцентный цвет (розовый)
        border: "#7640bd",    // Цвет для второстепенных элементов (тёмно-фиолетовый)
    },
    padding: Paddings {
        button: 2,
        border: 2,
        frame: 4,
    },
    gap: Gaps { frame: 4 }, // Увеличил gap для соответствия дизайну
});

pub static UMBRELLA_DARK: Shade<Theme> = Shade::Dark(Theme {
    color: Colors {
        primary: "#7f69b5",   // Основной цвет текста (светло-фиолетовый)
        secondary: "#371b1b", // Цвет фона (тёмно-коричневый)
        thirdly: "#de8cc5",   // Акцентный цвет (розовый)
        border: "#7640bd",    // Цвет для второстепенных элементов (тёмно-фиолетовый)
    },
    padding: Paddings {
        button: 2,
        border: 2,
        frame: 4,
    },
    gap: Gaps { frame: 4 }, // Увеличил gap для соответствия дизайну
});
