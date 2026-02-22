use std::fmt::Display;
use uuid::Uuid;

/// 承認申請ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApprovalRequestId(Uuid);

impl ApprovalRequestId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Display for ApprovalRequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
