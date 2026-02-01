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
        memberships: Vec<Membership>,
        group_type: GroupType,
    },
    Admin {
        user_id: UserId,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::clock::Clock;
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::Membership;
    use crate::domain::user_id::UserId;
    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    struct MockClock;
    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            Utc::now()
        }
    }

    #[test]
    fn test_is_group_type_project_general() {
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = ActorContext::User {
            user_id,
            memberships: vec![],
            group_type: GroupType::GeneralProject {
                representative1: user_id,
                representative2: user_id,
                representative3: user_id,
            },
        };
        assert!(ctx.is_group_type_project_general());
        assert!(!ctx.is_group_type_project_booth());
    }

    #[test]
    fn test_is_group_id() {
        let user_id = UserId::new(Uuid::new_v4());
        let group_id = GroupId::new('G', 1).unwrap();
        let memberships = vec![Membership::new(group_id, user_id, &MockClock)];
        let ctx = ActorContext::User {
            user_id,
            memberships,
            group_type: GroupType::Press {
                representative: user_id,
            },
        };
        assert!(ctx.is_group_id(&group_id));
        assert!(!ctx.is_group_id(&GroupId::new('G', 2).unwrap()));
    }

    #[test]
    fn test_is_user_id() {
        let user_id = UserId::new(Uuid::new_v4());
        let ctx_user = ActorContext::User {
            user_id,
            memberships: vec![],
            group_type: GroupType::Press {
                representative: user_id,
            },
        };
        let ctx_admin = ActorContext::Admin {
            user_id,
            claims: vec![],
        };
        let ctx_nologin = ActorContext::NoLogin;

        assert!(ctx_user.is_user_id(&user_id));
        assert!(ctx_admin.is_user_id(&user_id));
        assert!(!ctx_nologin.is_user_id(&user_id));
        assert!(!ctx_user.is_user_id(&UserId::new(Uuid::new_v4())));
    }

    #[test]
    fn test_is_user_nologin() {
        assert!(ActorContext::NoLogin.is_user_nologin());
        assert!(
            !ActorContext::Admin {
                user_id: UserId::new(Uuid::new_v4()),
                claims: vec![]
            }
            .is_user_nologin()
        );
    }
}
