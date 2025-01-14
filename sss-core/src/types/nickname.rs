use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Nickname {
    pub word: String,
    #[serde(default)]
    pub pronounce: String,
}
