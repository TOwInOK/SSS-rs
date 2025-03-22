#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};
    use theme::{Colors, Shade, Theme};
    use theme_generator::generate_theme;

    generate_theme! {
        fff {
            colors {
                text: "#000000",
                background: "#ffffff",
                accent: "#ff0000",
                border: "#cccccc",
            },
            font {
                name: "PT Mono",
                link: "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap"
            }
        }
        f2 {
            colors {
                text: "#ffffff",
                background: "#000000",
                accent: "#00ff00",
                border: "#0000ff",
            },
            font {
                name: "Roboto Mono",
                link: "https://fonts.googleapis.com/css2?family=Roboto+Mono&display=swap"
            }
        }
        f3 {
            colors {
                text: "#000000",
                background: "#ffffff",
                accent: "#0000ff",
                border: "#ff0000",
            },
            font {
                name: "Roboto Mono",
                link: "https://fonts.googleapis.com/css2?family=Roboto+Mono&display=swap"
            }
        }
    }

    #[test]
    fn check_vars() {
        // Colors
        assert_eq!("#000000", FFF.colors.text);
        assert_eq!("#ffffff", FFF.colors.background);
        assert_eq!("#ff0000", FFF.colors.accent);
        assert_eq!("#cccccc", FFF.colors.border);
        // Font
        assert_eq!("PT Mono", FFF.font.0);
        assert_eq!(
            "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap",
            FFF.font.1
        );
    }

    #[test]
    fn check_get_colors() {
        let colors = FFF.get_colors();
        assert_eq!(colors.text, "#000000");
        assert_eq!(colors.background, "#ffffff");
        assert_eq!(colors.accent, "#ff0000");
        assert_eq!(colors.border, "#cccccc");
    }

    #[test]
    fn check_get_font() {
        let font = FFF.font();
        assert_eq!(font.0, "PT Mono");
        assert_eq!(
            font.1,
            "https://fonts.googleapis.com/css2?family=PT+Mono&display=swap"
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(Themes::from_str("fff"), Ok(Themes::FFF));
        assert_eq!(Themes::from_str("FFF"), Ok(Themes::FFF));
    }
}
