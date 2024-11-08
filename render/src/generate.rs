use crate::theme::Shade;
use card::component::frame::Direction::*;
use card::component::text::Font::*;
use card::component::Component;

// generate.rs
pub fn gen(component: &Component, global_theme: &Shade) -> String {
    let theme = match global_theme {
        Shade::Dark(theme) | Shade::Light(theme) => theme,
    };
    match component {
        Component::Text(text) => match text.font {
            Label => format!(
                "<p class=\"text-lg font-semibold\" style=\"color: {}\">{}</p>",
                theme.primary,
                text.text
            ),
            SubLabel => format!(
                "<p class=\"text-md font-medium\" style=\"color: {}\">{}</p>",
                theme.secondary,
                text.text
            ),
            Text => format!(
                "<p class=\"text-sm\" style=\"color: {}\">{}</p>",
                theme.thirdly,
                text.text
            ),
            Minor => format!(
                "<p class=\"text-xs text-gray-500\">{}</p>",
                text.text
            ),
        },
        Component::Link(link) => format!(
            "<a href=\"{}\" class=\"text-blue-600 hover:text-blue-800\"><div>{}{}</div></a>",
            link.href, link.text.and_then(|x| Some(gen(&x, &global_theme))).unwrap_or_default(), link.icon.and_then(|x| Some(gen(&x, &global_theme))).unwrap_or_default()
        ),
        Component::Field(field) => format!(
            "<input type=\"text\" value=\"{}\" style=\"border-color: {}; padding: {}px;\" />",
            gen(field.title, global_theme) ,
            theme.border,
            theme.border_pudding
        ),
        Component::Icon(icon) => format!(
            "<i class=\"\" style=\"color: {};\">{}</i>",
            theme.primary,
            icon.as_str(),
        ),
        Component::Frame(frame) => match frame.direction {
            Vertical => format!(
                "<div class=\"flex flex-col\" style=\"border-color: {}; padding: {}px;\">{}</div>",
                theme.border,
                theme.frame_pudding,
                frame_gen(frame.data, global_theme)
            ),
            Horizontal => format!(
                "<div class=\"flex\" style=\"border-color: {}; padding: {}px;\">{}</div>",
                theme.border,
                theme.frame_pudding,
                frame_gen(frame.data, global_theme)
            ),
            ReversVertical => format!(
                "<div class=\"flex flex-col-reverse\" style=\"border-color: {}; padding: {}px;\">{}</div>",
                theme.border,
                theme.frame_pudding,
                frame_gen(frame.data, global_theme)
            ),
            ReversHorizontal => format!(
                "<div class=\"flex flex-row-reverse\" style=\"border-color: {}; padding: {}px;\">{}</div>",
                theme.border,
                theme.frame_pudding,
                frame_gen(frame.data, global_theme)
            ),
        },
    }
}

fn frame_gen(frame: &[Component<'_>], theme: &Shade) -> String {
    frame.iter().map(|x| gen(x, &theme)).collect::<String>()
}
