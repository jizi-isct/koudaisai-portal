use crate::entities::user::UserCreate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaboCreate {
    pub representative: UserCreate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaboRead {
    pub representative: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaboUpdate {
    #[serde(default)]
    pub representative: Option<Uuid>,
}
