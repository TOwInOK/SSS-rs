use serde::{Deserialize, Serialize};
use types::user::User;

pub mod types;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub user: User,
}
