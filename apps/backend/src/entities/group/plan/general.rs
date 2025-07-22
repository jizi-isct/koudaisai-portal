use crate::entities::user::UserCreate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralCreate {
    pub(crate) representative1: UserCreate,
    pub(crate) representative2: UserCreate,
    pub(crate) representative3: UserCreate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralRead {
    pub(crate) representative1: Uuid,
    pub(crate) representative2: Uuid,
    pub(crate) representative3: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralUpdate {
    #[serde(default)]
    pub(crate) representative1: Option<Uuid>,
    #[serde(default)]
    pub(crate) representative2: Option<Uuid>,
    #[serde(default)]
    pub(crate) representative3: Option<Uuid>,
}
