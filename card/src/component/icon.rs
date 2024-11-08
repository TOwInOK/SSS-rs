#[macro_export]
/// Return svg with under variant via name
/// Variants:
/// - Outline
/// - Filled
/// - Custom (push &str)
macro_rules! create_icon {
    ($t:ident, $item:ident) => {
        Icon::$t($t::$item)
    };
    ($t:ident, $item:expr) => {
        Icon::$t($item)
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Icon<'a> {
    Filled(Filled),
    Outline(Outline),
    Custom(&'a str),
}

impl Icon<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            Icon::Filled(filled) => filled.as_str(),
            Icon::Outline(outline) => outline.as_str(),
            Icon::Custom(e) => e,
        }
    }
}
/// Gen outline svg
macro_rules! icon_outline {
    ($($name:ident($svg:expr)),+ $(,)?) => {

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Outline {
            $($name),*
        }

        impl Outline {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                       Outline::$name => $svg
                    ),*
                }
            }
        }
    };
}
/// Gen filled svg
macro_rules! icon_filled {
    ($($name:ident($svg:expr)),+ $(,)?) => {

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Filled {
            $($name),*
        }
        impl Filled {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        Filled::$name => $svg
                     ),*
                }
            }
        }
    };
}

icon_filled!(
    GitHub(
        r#"<svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-brand-github"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5" /></svg>"#
    ),
    GitHub2(
        r#"<svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-brand-github"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5" /></svg>"#
    )
);
icon_outline!(GitHub(
    r#"<svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-brand-github"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5" /></svg>"#
));
