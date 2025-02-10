use serde::{Deserialize, Serialize};

use super::provider::Tabler;

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Link {
    pub icon: Tabler,
    pub link: String,
}
