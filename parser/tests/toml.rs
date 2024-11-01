#[cfg(test)]
mod toml {
    use parser::{parse::parse_toml, types::User};
    use std::fs;

    fn parse_default_toml() {
        parse_toml(None).expect("Failed to parse default TOML");
    }

    fn generate_default_toml() {
        let user = User::default();
        let toml = toml::to_string_pretty(&user).expect("Failed to convert user to TOML");
        fs::write("./example.toml", toml).expect("Failed to write example.toml");
    }

    fn generate_scheme_restyle_toml() {
        let user = parse_toml(None).expect("Failed to parse default TOML");
        let toml = toml::to_string(&user).expect("Failed to convert user to TOML");
        fs::write("./example.toml", toml).expect("Failed to write example.toml");
    }

    fn parse_custom_toml() {
        parse_toml(Some("./user.toml")).expect("Failed to parse custom TOML");
    }

    #[test]
    fn toml_test() {
        parse_default_toml();
        generate_default_toml();
        generate_scheme_restyle_toml();
        parse_custom_toml();
    }
}
