use serde::{Deserialize, Serialize};

use super::provider::Provider;

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Link {
    pub provider: Provider,
    pub link: String,
}
