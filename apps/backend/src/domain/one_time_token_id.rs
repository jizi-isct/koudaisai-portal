use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
pub struct OneTimeTokenId(Uuid);

impl OneTimeTokenId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl Display for OneTimeTokenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<OneTimeTokenId> for Uuid {
    fn from(id: OneTimeTokenId) -> Self {
        id.0
    }
}
