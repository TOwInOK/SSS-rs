use crate::theme::Shade;
use card::component::frame::Direction::*;
use card::component::text::Font::*;
use card::component::Component;
use leptos::IntoView;

/// Generate raw html string
pub fn gen_html(component: &Component, global_theme: &Shade) -> String {
    let theme = match global_theme {
        Shade::Dark(theme) | Shade::Light(theme) => theme,
    };
    match component {
        Component::Text(text) => match text.font {
            Label => format!(
                "<p class=\"{}\">
                    {}
                </p>",
                theme.label(),
                text.text
            ),
            SubLabel => format!(
                "<p class=\"{}\">
                    {}
                </p>",
                theme.sub_label(),
                text.text
            ),
            Text => format!(
                "<p class=\"{}\">
                    {}
                </p>",
                theme.text(),
                text.text
            ),
            Minor => format!(
                "<p class=\"{}\">
                    {}
                </p>",
                theme.text_minor(),
                text.text
            ),
        },
        Component::Link(link) => format!(
            "<a href=\"{}\" class=\"{}\">
                {}{}
            </a>",
            link.href,
            theme.link(),
            link.text
                .and_then(|x| Some(gen_html(&x, &global_theme)))
                .unwrap_or_default(),
            link.icon
                .and_then(|x| Some(gen_html(&x, &global_theme)))
                .unwrap_or_default()
        ),
        Component::Field(field) => format!(
            "<input type=\"text\" value=\"{}\" class=\"{}\" />",
            gen_html(field.title, global_theme),
            theme.field(),
        ),
        Component::Icon(icon) => format!(
            "<i class=\"{}\">
                {}
            </i>",
            theme.icon(),
            icon.as_str(),
        ),
        Component::Frame(frame) => match frame.direction {
            Vertical => format!(
                "<div class=\"{}\">
                    {}
                </div>",
                theme.vertical_frame(),
                frame_gen_html(frame.data, global_theme)
            ),
            Horizontal => format!(
                "<div class=\"{}\">
                    {}
                </div>",
                theme.horizontal_frame(),
                frame_gen_html(frame.data, global_theme)
            ),
            ReversVertical => format!(
                "<div class=\"{}\">
                    {}
                </div>",
                theme.reversed_vertical_frame(),
                frame_gen_html(frame.data, global_theme)
            ),
            ReversHorizontal => format!(
                "<div class=\"{}\">
                    {}
                </div>",
                theme.reversed_horizontal_frame(),
                frame_gen_html(frame.data, global_theme)
            ),
        },
    }
}

fn frame_gen_html(frame: &[Component<'_>], theme: &Shade) -> String {
    frame
        .iter()
        .map(|x| gen_html(x, &theme))
        .collect::<String>()
}
// fn frame_gen_leptos<'a, 'b>(frame: &'a [Component<'a>], theme: &'b Shade) -> Vec<impl IntoView> {
//     frame
//         .iter()
//         .map(|x| gen_leptos_component(x, theme))
//         .collect::<Vec<_>>()
// }

// #[component]
// pub fn gen_leptos_component<'a>(
//     component: &'a Component<'a>,
//     global_theme: &'a Shade,
// ) -> impl IntoView {
//     let theme = match global_theme {
//         Shade::Dark(theme) | Shade::Light(theme) => theme,
//     };

//     let result = match component {
//         Component::Text(text) => {
//             let class = match text.font {
//                 Label => theme.text(),
//                 SubLabel => theme.sub_label(),
//                 Text => theme.text(),
//                 Minor => theme.text_minor(),
//             };
//             // Создаем единый View для текста
//             view! { <p class={class}>{text.text}</p> }
//         }
//         Component::Frame(frame) => {
//             let frame_class = match frame.direction {
//                 Vertical => theme.vertical_frame(),
//                 Horizontal => theme.horizontal_frame(),
//                 ReversVertical => theme.reversed_vertical_frame(),
//                 ReversHorizontal => theme.reversed_horizontal_frame(),
//             };

//             let children = frame_gen_leptos(frame.data, global_theme);

//             // Создаем единый View для Frame
//             view! {
//                 <div class={frame_class}>
//                     {children.into_view()}
//                 </div>
//             }
//         }
//         _ => view! { <p>{"Component not implemented yet!"}</p> },
//     };

//     // Приводим результат к типу View
//     result.into_view()
// }
