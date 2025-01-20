use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Since {
    pub start: usize,
    #[serde(default)]
    pub end: usize,
}

impl std::fmt::Display for Since {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.end == 0 {
            write!(f, "{} -> today", self.start)
        } else {
            write!(f, "{} -> {}", self.start, self.end)
        }
    }
}
