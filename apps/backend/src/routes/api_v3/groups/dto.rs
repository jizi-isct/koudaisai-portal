use crate::domain::group::{Group, GroupType as DomainGroupType};
use crate::domain::membership::{Membership, Role as DomainRole};
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

impl From<&DomainGroupType> for GroupType {
    fn from(g: &DomainGroupType) -> Self {
        match g {
            DomainGroupType::Press => GroupType::Press,
            DomainGroupType::GeneralProject => GroupType::GeneralProject,
            DomainGroupType::BoothProject => GroupType::BoothProject,
            DomainGroupType::LabProject => GroupType::LabProject,
            DomainGroupType::StageProject => GroupType::StageProject,
        }
    }
}

impl From<&GroupType> for DomainGroupType {
    fn from(g: &GroupType) -> Self {
        match g {
            GroupType::Press => DomainGroupType::Press,
            GroupType::GeneralProject => DomainGroupType::GeneralProject,
            GroupType::BoothProject => DomainGroupType::BoothProject,
            GroupType::LabProject => DomainGroupType::LabProject,
            GroupType::StageProject => DomainGroupType::StageProject,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct GroupCreate {
    pub name: String,
    pub r#type: GroupType,
}

#[derive(Serialize, ToSchema)]
pub struct GroupRead {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub r#type: GroupType,
}

impl From<&Group> for GroupRead {
    fn from(g: &Group) -> Self {
        GroupRead {
            id: g.id().to_string(),
            created_at: g.created_at(),
            updated_at: g.updated_at(),
            name: g.name().to_string(),
            r#type: g.r#type().into(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct GroupUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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

impl From<&DomainRole> for Role {
    fn from(r: &DomainRole) -> Self {
        match r {
            DomainRole::Representative => Role::Representative,
            DomainRole::Operator => Role::Operator,
            DomainRole::FirstResponsible => Role::FirstResponsible,
            DomainRole::SecondResponsible => Role::SecondResponsible,
            DomainRole::ThirdResponsible => Role::ThirdResponsible,
        }
    }
}

impl From<&Role> for DomainRole {
    fn from(r: &Role) -> Self {
        match r {
            Role::Representative => DomainRole::Representative,
            Role::Operator => DomainRole::Operator,
            Role::FirstResponsible => DomainRole::FirstResponsible,
            Role::SecondResponsible => DomainRole::SecondResponsible,
            Role::ThirdResponsible => DomainRole::ThirdResponsible,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct MemberRead {
    pub user_id: Uuid,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}

impl From<&Membership> for MemberRead {
    fn from(m: &Membership) -> Self {
        MemberRead {
            user_id: m.user_id().into(),
            role: (&m.role()).into(),
            created_at: m.created_at(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct MemberCreate {
    pub user_id: Uuid,
}
