use crate::domain::actor_ctx::ActorContext;
use crate::domain::error::FactoryError;
use crate::domain::group_id::GroupId;
use crate::domain::user_id::UserId;
use std::str::FromStr;
use uuid::Uuid;

/// ユーザーの認可に使われる表示対象を指定するための列挙型
#[derive(Debug, Clone, PartialEq)]
pub enum TargetSpecifier {
    GroupTypeProjectGeneral,
    GroupTypeProjectBooth,
    GroupTypeProjectStage,
    GroupTypeProjectLabo,
    GroupTypePress,
    GroupId(GroupId),
    UserId(UserId),
    UserNologin,
}

impl FromStr for TargetSpecifier {
    type Err = FactoryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let target_specifier = match s {
            "group/type/project_general" => TargetSpecifier::GroupTypeProjectGeneral,
            "group/type/project_booth" => TargetSpecifier::GroupTypeProjectBooth,
            "group/type/project_stage" => TargetSpecifier::GroupTypeProjectStage,
            "group/type/project_labo" => TargetSpecifier::GroupTypeProjectLabo,
            "group/type/press" => TargetSpecifier::GroupTypePress,
            "user/nologin" => TargetSpecifier::UserNologin,
            s if s.starts_with("user/id/") => {
                let user_id_str = &s[8..];
                match Uuid::parse_str(user_id_str) {
                    Ok(uuid) => TargetSpecifier::UserId(UserId::new(uuid)),
                    Err(_) => {
                        return Err(FactoryError::InvalidInput(format!("Invalid uuid: {}", s)));
                    }
                }
            }
            s if s.starts_with("group/id/") => {
                let group_id = GroupId::from_str(&s[9..])?;
                TargetSpecifier::GroupId(group_id)
            }
            _ => {
                return Err(FactoryError::InvalidInput(format!(
                    "Invalid target specifier: {}",
                    s
                )));
            }
        };
        Ok(target_specifier)
    }
}

impl Into<String> for &TargetSpecifier {
    fn into(self) -> String {
        match self {
            TargetSpecifier::GroupTypeProjectGeneral => "group/type/project_general".to_string(),
            TargetSpecifier::GroupTypeProjectBooth => "group/type/project_booth".to_string(),
            TargetSpecifier::GroupTypeProjectStage => "group/type/project_stage".to_string(),
            TargetSpecifier::GroupTypeProjectLabo => "group/type/project_labo".to_string(),
            TargetSpecifier::GroupTypePress => "group/type/press".to_string(),
            TargetSpecifier::GroupId(id) => format!("group/id/{}", id),
            TargetSpecifier::UserId(user_id) => format!("user/id/{}", user_id),
            TargetSpecifier::UserNologin => "user/nologin".to_string(),
        }
    }
}

impl TargetSpecifier {
    pub fn does_actor_match(&self, actor_ctx: &ActorContext) -> bool {
        match self {
            TargetSpecifier::GroupTypeProjectGeneral => actor_ctx.is_group_type_project_general(),
            TargetSpecifier::GroupTypeProjectBooth => actor_ctx.is_group_type_project_booth(),
            TargetSpecifier::GroupTypeProjectStage => actor_ctx.is_group_type_project_stage(),
            TargetSpecifier::GroupTypeProjectLabo => actor_ctx.is_group_type_project_labo(),
            TargetSpecifier::GroupTypePress => actor_ctx.is_group_type_press(),
            TargetSpecifier::GroupId(group_id) => actor_ctx.is_group_id(group_id),
            TargetSpecifier::UserId(user_id) => actor_ctx.is_user_id(user_id),
            TargetSpecifier::UserNologin => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::clock::Clock;
    use crate::domain::group::GroupType;
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
    fn test_does_actor_match() {
        let user_id = UserId::new(Uuid::new_v4());
        let group_id = GroupId::from_str("M-001").unwrap();
        let memberships = vec![Membership::new(group_id, user_id, &MockClock)];

        let ctx_general = ActorContext::User {
            user_id,
            memberships: memberships.clone(),
            group_type: GroupType::GeneralProject {
                representative1: user_id,
                representative2: user_id,
                representative3: user_id,
            },
        };

        let ctx_nologin = ActorContext::NoLogin;

        // GroupTypeProjectGeneral
        assert!(TargetSpecifier::GroupTypeProjectGeneral.does_actor_match(&ctx_general));
        assert!(!TargetSpecifier::GroupTypeProjectGeneral.does_actor_match(&ctx_nologin));

        // GroupId
        assert!(TargetSpecifier::GroupId(group_id).does_actor_match(&ctx_general));
        assert!(
            !TargetSpecifier::GroupId(GroupId::from_str("M-002").unwrap())
                .does_actor_match(&ctx_general)
        );

        // UserId
        assert!(TargetSpecifier::UserId(user_id).does_actor_match(&ctx_general));
        assert!(
            !TargetSpecifier::UserId(UserId::new(Uuid::new_v4())).does_actor_match(&ctx_general)
        );

        // UserNologin
        assert!(TargetSpecifier::UserNologin.does_actor_match(&ctx_nologin));
        assert!(!TargetSpecifier::UserNologin.does_actor_match(&ctx_general));
    }
}
