use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct FormId(Uuid);

impl FormId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl Display for FormId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
