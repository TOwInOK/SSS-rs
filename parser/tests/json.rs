#[cfg(test)]
mod json {
    use parser::{parse::parse_json, parse::parse_toml, types::User};
    use std::fs;

    fn parse_default_json() {
        parse_json(None).expect("Failed to parse default TOML");
    }

    fn generate_default_json() {
        let user = User::default();
        let toml = serde_json::to_string_pretty(&user).expect("Failed to convert user to TOML");
        fs::write("./example.json", toml).expect("Failed to write example.toml");
    }

    fn generate_scheme_restyle_json() {
        let user = parse_toml(None).expect("Failed to parse default TOML");
        let toml = serde_json::to_string(&user).expect("Failed to convert user to TOML");
        fs::write("./example.json", toml).expect("Failed to write example.toml");
    }

    fn parse_custom_json() {
        parse_json(Some("./user.json")).expect("Failed to parse custom TOML");
    }

    #[test]
    fn json_test() {
        parse_default_json();
        generate_default_json();
        generate_scheme_restyle_json();
        parse_custom_json();
    }
}
