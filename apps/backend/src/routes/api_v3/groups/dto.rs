use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum GroupType {
    Press,
    GeneralProject,
    BoothProject,
    LabProject,
    StageProject,
}

#[derive(Deserialize, ToSchema)]
pub struct GroupCreate {
    name: String,
    r#type: GroupType,
}

#[derive(Serialize, ToSchema)]
pub struct GroupRead {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    r#type: GroupType,
}

#[derive(Deserialize, ToSchema)]
pub struct GroupUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Representative,
    Operator,
    FirstResponsible,
    SecondResponsible,
    ThirdResponsible,
}

#[derive(Serialize, ToSchema)]
pub struct MemberRead {
    user_id: Uuid,
    role: Role,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct MemberCreate {
    user_id: Uuid,
}
