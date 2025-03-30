use link::Link;
use nickname::Nickname;
use since::Since;
use skill::Skill;
use user::User;

use crate::LayoutLimitations;

pub mod link;
pub mod nickname;
pub mod provider;
pub mod since;
pub mod skill;
pub mod user;

pub fn user() -> User {
    User::default()
}

pub fn nickname() -> Nickname {
    Nickname::default()
}

pub fn blank() -> Link {
    Link::default()
}

pub fn skill() -> Skill {
    Skill::default()
}

pub fn since() -> Since {
    Since::default()
}

pub fn limitations() -> LayoutLimitations {
    LayoutLimitations::default()
}
