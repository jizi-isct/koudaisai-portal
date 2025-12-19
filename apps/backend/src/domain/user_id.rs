use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct UserId {
    pub id: Uuid
}

impl UserId {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}