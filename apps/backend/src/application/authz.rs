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
            if (claims.contains(&"koudaisai-portal:admin:user:read".to_string())) {
                Ok(())
            } else {
                Err(CanGetByIdError::Unauthorized)
            }
        }
        ActorContext::User { memberships, .. } => {
            // 自分自身の情報、もしくは同じグループに所属しているユーザーの情報は取得可能
            if (memberships.iter().any(|m| {
                memberships_of_the_user
                    .iter()
                    .any(|m2| m.group_id() == m2.group_id())
            })) {
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
        },
        _ => false
    }
}

pub fn can_change_m_address_of_the_user(actor_ctx: &ActorContext, user_id: UserId) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:user:change-email".to_string())
        },
        _ => false
    }
}

pub fn can_get_all_groups(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:group:read".to_string())
        },
        _ => false
    }
}

pub fn can_get_group_by_id(actor_ctx: &ActorContext, members: &Vec<Membership>) -> Result<(), CanGetByIdError> {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            if claims.contains(&"koudaisai-portal:admin:group:read".to_string()) {
                Ok(())
            } else {
                Err(CanGetByIdError::Unauthorized)
            }
        },
        ActorContext::User { user_id, .. } => {
            // ユーザーがグループに所属しているかどうかを確認
            if members.iter().any(|m| m.user_id() == *user_id) {
                Ok(())
            } else {
                Err(CanGetByIdError::NotFound)
            }
        },
        ActorContext::NoLogin => Err(CanGetByIdError::Unauthorized)
    }
}

pub fn can_create_group(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:group:create".to_string())
        },
        _ => false
    }
}