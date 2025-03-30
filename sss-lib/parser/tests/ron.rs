#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

    use parser::parse::parse;
    use sss_core::LayoutLimitations;

    #[test]
    fn parse_limitations() {
        let limitations: LayoutLimitations = parse(Path::new("limit.ron")).unwrap();
        println!("{:?}", limitations);
    }

    #[test]
    fn parse_limitations_macros() {
        let readed = read_to_string("limit.ron").unwrap();
        let limitations: LayoutLimitations = ron::from_str(&readed).unwrap();
        println!("{:?}", limitations);
    }
}
