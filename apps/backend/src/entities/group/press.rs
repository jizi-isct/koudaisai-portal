use crate::entities::user::UserCreate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressCreate {
    pub representative: UserCreate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressRead {
    pub representative: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressUpdate {
    #[serde(default)]
    pub representative: Option<Uuid>,
}
