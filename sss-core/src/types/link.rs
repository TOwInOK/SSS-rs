use serde::{Deserialize, Serialize};

use super::provider::Provider;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Link {
    pub provider: Provider,
    pub link: String,
}
