use crate::domain::group::GroupType;
use crate::domain::group_id::GroupId;
use crate::domain::membership::Membership;
use crate::domain::user_id::UserId;

/// 認可に関するモデル
/// 認可に関するコンテキストを保持するモデルです。
/// ユーザー、管理者、未ログイン状態を表現します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorContext {
    User {
        user_id: UserId,
        /// 表示名(通知の送信者名などに用いる)。
        name: String,
        memberships: Vec<Membership>,
        group_type: GroupType,
    },
    Admin {
        user_id: UserId,
        /// 表示名(通知の送信者名などに用いる)。
        name: String,
        claims: Vec<String>,
    },
    NoLogin,
}

impl ActorContext {
    pub fn is_group_type_project_general(&self) -> bool {
        match self {
            ActorContext::User { group_type, .. } => {
                matches!(group_type, GroupType::GeneralProject { .. })
            }
            _ => false,
        }
    }

    pub fn is_group_type_project_booth(&self) -> bool {
        match self {
            ActorContext::User { group_type, .. } => {
                matches!(group_type, GroupType::BoothProject { .. })
            }
            _ => false,
        }
    }

    pub fn is_group_type_project_stage(&self) -> bool {
        match self {
            ActorContext::User { group_type, .. } => {
                matches!(group_type, GroupType::StageProject { .. })
            }
            _ => false,
        }
    }

    pub fn is_group_type_project_labo(&self) -> bool {
        match self {
            ActorContext::User { group_type, .. } => {
                matches!(group_type, GroupType::LabProject { .. })
            }
            _ => false,
        }
    }

    pub fn is_group_type_press(&self) -> bool {
        match self {
            ActorContext::User { group_type, .. } => {
                matches!(group_type, GroupType::Press { .. })
            }
            _ => false,
        }
    }

    pub fn is_group_id(&self, group_id: &GroupId) -> bool {
        match self {
            ActorContext::User { memberships, .. } => {
                memberships.iter().any(|m| m.group_id() == *group_id)
            }
            _ => false,
        }
    }

    pub fn is_user_id(&self, user_id: &UserId) -> bool {
        match self {
            ActorContext::User {
                user_id: actor_user_id,
                ..
            } => actor_user_id == user_id,
            ActorContext::Admin {
                user_id: actor_user_id,
                ..
            } => actor_user_id == user_id,
            ActorContext::NoLogin => false,
        }
    }

    pub fn is_user_nologin(&self) -> bool {
        matches!(self, ActorContext::NoLogin)
    }

    pub fn user_id(&self) -> Option<UserId> {
        match self {
            ActorContext::User { user_id, .. } => Some(*user_id),
            ActorContext::Admin { user_id, .. } => Some(*user_id),
            ActorContext::NoLogin => None,
        }
    }
}
