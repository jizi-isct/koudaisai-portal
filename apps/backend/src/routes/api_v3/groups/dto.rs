use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum GroupType {
    TypePress {
        representative: Uuid,
    },
    TypeGeneralProject {
        representative1: Uuid,
        representative2: Uuid,
        representative3: Uuid,
    },
    TypeBoothProject {
        representative1: Uuid,
        representative2: Uuid,
        representative3: Uuid,
    },
    TypeLabProject {
        representative: Uuid,
        operator: Uuid,
    },
    TypeStageProject {
        representative1: Uuid,
        representative2: Uuid,
        representative3: Uuid,
    },
}

#[derive(Deserialize, ToSchema)]
pub struct GroupCreate {
    name: String,
    #[serde(flatten)]
    r#type: GroupType,
}

#[derive(Serialize, ToSchema)]
pub struct GroupRead {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    #[serde(flatten)]
    r#type: GroupType,
}

#[derive(Deserialize, ToSchema)]
pub struct GroupUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
