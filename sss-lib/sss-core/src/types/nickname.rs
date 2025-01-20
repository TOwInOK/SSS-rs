use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Nickname {
    pub word: String,
    #[serde(default)]
    pub pronounce: String,
}
