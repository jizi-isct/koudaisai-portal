use crate::domain::actor_ctx::ActorContext;
use crate::domain::membership::Membership;
use crate::domain::user::User;
use crate::domain::user_id::UserId;

pub fn can_get_all_users(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:user:read".to_string())
        }
        _ => false,
    }
}

pub enum CanGetByIdError {
    NotFound,
    Unauthorized,
}

pub fn can_get_user_by_id(
    actor_ctx: &ActorContext,
    memberships_of_the_user: Vec<Membership>,
) -> Result<(), CanGetByIdError> {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            if claims.contains(&"koudaisai-portal:admin:user:read".to_string()) {
                Ok(())
            } else {
                Err(CanGetByIdError::Unauthorized)
            }
        }
        ActorContext::User { memberships, .. } => {
            // 自分自身の情報、もしくは同じグループに所属しているユーザーの情報は取得可能
            if memberships.iter().any(|m| {
                memberships_of_the_user
                    .iter()
                    .any(|m2| m.group_id() == m2.group_id())
            }) {
                Ok(())
            } else {
                Err(CanGetByIdError::NotFound)
            }
        }
        ActorContext::NoLogin => Err(CanGetByIdError::Unauthorized),
    }
}

pub fn can_update_user(actor_ctx: &ActorContext, user: &User) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:user:update".to_string())
        }
        _ => false,
    }
}

pub fn can_change_m_address_of_the_user(actor_ctx: &ActorContext, user_id: UserId) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:user:change-email".to_string())
        }
        _ => false,
    }
}

pub fn can_get_all_groups(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:group:read".to_string())
        }
        _ => false,
    }
}

pub fn can_get_group_by_id(
    actor_ctx: &ActorContext,
    members: &Vec<Membership>,
) -> Result<(), CanGetByIdError> {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            if claims.contains(&"koudaisai-portal:admin:group:read".to_string()) {
                Ok(())
            } else {
                Err(CanGetByIdError::Unauthorized)
            }
        }
        ActorContext::User { user_id, .. } => {
            // ユーザーがグループに所属しているかどうかを確認
            if members.iter().any(|m| m.user_id() == *user_id) {
                Ok(())
            } else {
                Err(CanGetByIdError::NotFound)
            }
        }
        ActorContext::NoLogin => Err(CanGetByIdError::Unauthorized),
    }
}

pub fn can_create_group(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:group:create".to_string())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::clock::Clock;
    use crate::domain::actor_ctx::ActorContext;
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

    fn create_user_id() -> UserId {
        UserId::new(Uuid::new_v4())
    }

    fn create_group_id() -> GroupId {
        GroupId::new('G', 1).unwrap()
    }

    fn create_membership(group_id: GroupId, user_id: UserId) -> Membership {
        Membership::new(group_id, user_id, &MockClock)
    }

    #[test]
    fn test_can_get_all_users() {
        let admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:user:read".to_string()],
        };
        assert!(can_get_all_users(&admin_ctx));

        let unauthorized_admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_get_all_users(&unauthorized_admin_ctx));

        let user_ctx = ActorContext::User {
            user_id: create_user_id(),
            memberships: vec![],
            group_type: GroupType::Press {
                representative: create_user_id(),
            },
        };
        assert!(!can_get_all_users(&user_ctx));

        assert!(!can_get_all_users(&ActorContext::NoLogin));
    }

    #[test]
    fn test_can_get_user_by_id() {
        let user_id = create_user_id();
        let group_id = create_group_id();
        let membership = create_membership(group_id, user_id);
        let memberships_of_the_user = vec![membership.clone()];

        // Admin with permission
        let admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:user:read".to_string()],
        };
        assert!(can_get_user_by_id(&admin_ctx, memberships_of_the_user.clone()).is_ok());

        // Admin without permission
        let unauthorized_admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(matches!(
            can_get_user_by_id(&unauthorized_admin_ctx, memberships_of_the_user.clone()),
            Err(CanGetByIdError::Unauthorized)
        ));

        // User in the same group
        let other_user_id = create_user_id();
        let same_group_user_ctx = ActorContext::User {
            user_id: other_user_id,
            memberships: vec![create_membership(group_id, other_user_id)],
            group_type: GroupType::Press {
                representative: other_user_id,
            },
        };
        assert!(can_get_user_by_id(&same_group_user_ctx, memberships_of_the_user.clone()).is_ok());

        // User in a different group
        let diff_group_id = GroupId::new('G', 2).unwrap();
        let diff_group_user_ctx = ActorContext::User {
            user_id: other_user_id,
            memberships: vec![create_membership(diff_group_id, other_user_id)],
            group_type: GroupType::Press {
                representative: other_user_id,
            },
        };
        assert!(matches!(
            can_get_user_by_id(&diff_group_user_ctx, memberships_of_the_user.clone()),
            Err(CanGetByIdError::NotFound)
        ));

        // NoLogin
        assert!(matches!(
            can_get_user_by_id(&ActorContext::NoLogin, memberships_of_the_user.clone()),
            Err(CanGetByIdError::Unauthorized)
        ));
    }

    #[test]
    fn test_can_update_user() {
        // User entity is not actually used in the implementation, but we need it for the signature
        use crate::domain::email_address::EmailAddress;
        use crate::domain::user::User;
        let clock = MockClock;
        let user = User::register(
            create_user_id(),
            "test".to_string(),
            EmailAddress::new("test@example.com".to_string()).unwrap(),
            clock,
        )
        .unwrap();

        let admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:user:update".to_string()],
        };
        assert!(can_update_user(&admin_ctx, &user));

        let unauthorized_admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_update_user(&unauthorized_admin_ctx, &user));

        assert!(!can_update_user(&ActorContext::NoLogin, &user));
    }

    #[test]
    fn test_can_change_m_address_of_the_user() {
        let user_id = create_user_id();

        let admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:user:change-email".to_string()],
        };
        assert!(can_change_m_address_of_the_user(&admin_ctx, user_id));

        let unauthorized_admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_change_m_address_of_the_user(
            &unauthorized_admin_ctx,
            user_id
        ));

        assert!(!can_change_m_address_of_the_user(
            &ActorContext::NoLogin,
            user_id
        ));
    }

    #[test]
    fn test_can_get_all_groups() {
        let admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:group:read".to_string()],
        };
        assert!(can_get_all_groups(&admin_ctx));

        let unauthorized_admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_get_all_groups(&unauthorized_admin_ctx));

        assert!(!can_get_all_groups(&ActorContext::NoLogin));
    }

    #[test]
    fn test_can_get_group_by_id() {
        let group_id = create_group_id();
        let user_id = create_user_id();
        let members = vec![create_membership(group_id, user_id)];

        // Admin with permission
        let admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:group:read".to_string()],
        };
        assert!(can_get_group_by_id(&admin_ctx, &members).is_ok());

        // Admin without permission
        let unauthorized_admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(matches!(
            can_get_group_by_id(&unauthorized_admin_ctx, &members),
            Err(CanGetByIdError::Unauthorized)
        ));

        // User who is a member
        let member_user_ctx = ActorContext::User {
            user_id,
            memberships: vec![create_membership(group_id, user_id)],
            group_type: GroupType::Press {
                representative: user_id,
            },
        };
        assert!(can_get_group_by_id(&member_user_ctx, &members).is_ok());

        // User who is not a member
        let other_user_id = create_user_id();
        let non_member_user_ctx = ActorContext::User {
            user_id: other_user_id,
            memberships: vec![],
            group_type: GroupType::Press {
                representative: other_user_id,
            },
        };
        assert!(matches!(
            can_get_group_by_id(&non_member_user_ctx, &members),
            Err(CanGetByIdError::NotFound)
        ));

        // NoLogin
        assert!(matches!(
            can_get_group_by_id(&ActorContext::NoLogin, &members),
            Err(CanGetByIdError::Unauthorized)
        ));
    }

    #[test]
    fn test_can_create_group() {
        let admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:group:create".to_string()],
        };
        assert!(can_create_group(&admin_ctx));

        let unauthorized_admin_ctx = ActorContext::Admin {
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_create_group(&unauthorized_admin_ctx));

        assert!(!can_create_group(&ActorContext::NoLogin));
    }
}
