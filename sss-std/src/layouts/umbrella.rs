pub mod html;
pub mod html_tera;
pub mod img;
pub mod leptos;
pub mod pdf;
pub use self::{
    html::UmbrellaHtmlRender, html_tera::UmbrellaHtmlTeraRender, img::UmbrellaImgRender,
    leptos::UmbrellaLeptosRender, pdf::UmbrellaPdfRender,
};
