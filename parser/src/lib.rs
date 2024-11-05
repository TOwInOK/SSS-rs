pub mod error;
pub mod parse;

pub mod prelude {
    pub use crate::parse::parse_json;
    pub use crate::parse::parse_toml;
}
