pub mod html;
pub mod img;
pub mod leptos;
pub mod pdf;
pub use self::{
    html::UmbrellaHtmlRender, img::UmbrellaImgRender, leptos::UmbrellaLeptosRender,
    pdf::UmbrellaPdfRender,
};
use render::prelude::*;

pub static UMBRELLA: Theme = Theme {
    color: Colors {
        primary: "#7f69b5",   // Основной цвет текста (светло-фиолетовый)
        secondary: "#371b1b", // Цвет фона (тёмно-коричневый)
        thirdly: "#de8cc5",   // Акцентный цвет (розовый)
        border: "#7640bd",    // Цвет для второстепенных элементов (тёмно-фиолетовый)
    },
};
