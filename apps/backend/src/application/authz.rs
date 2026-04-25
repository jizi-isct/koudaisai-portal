use crate::domain::actor_ctx::ActorContext;
use crate::domain::document::Document;
use crate::domain::form::Form;
use crate::domain::group::Group;
use crate::domain::membership::Membership;
use crate::domain::user::User;
use crate::domain::user_id::UserId;
use uuid::Uuid;

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

pub fn can_get_form(actor_ctx: &ActorContext, form: &Form) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:form:read".to_string())
        }
        _ => form.targets().iter().any(|t| t.does_actor_match(actor_ctx)),
    }
}

pub fn can_create_form(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:form:create".to_string())
        }
        _ => false,
    }
}

pub fn can_update_form(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:form:update".to_string())
        }
        _ => false,
    }
}

pub fn can_delete_form(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:form:delete".to_string())
        }
        _ => false,
    }
}

// ============================================================
// 承認申請（ApprovalRequest）の認可ルール
// ============================================================

use crate::domain::approval_request::ApprovalRequest;
use crate::domain::group_id::GroupId;

/// 全ての承認申請を取得できるか（管理者用）
pub fn can_get_all_approval_requests(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:approval-request:read".to_string())
        }
        _ => false,
    }
}

/// グループメンバーの承認申請を取得できるか
pub fn can_get_group_approval_requests(actor_ctx: &ActorContext, group_id: GroupId) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:approval-request:read".to_string())
        }
        ActorContext::User { memberships, .. } => {
            // 同じグループに所属している場合のみ取得可能
            memberships.iter().any(|m| m.group_id() == group_id)
        }
        ActorContext::NoLogin => false,
    }
}

/// 特定の承認申請を取得できるか
pub fn can_get_approval_request(
    actor_ctx: &ActorContext,
    request: &ApprovalRequest,
    memberships_of_issuer: &[Membership],
) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:approval-request:read".to_string())
        }
        ActorContext::User { memberships, .. } => {
            // 同じグループに所属しているユーザーの申請は閲覧可能
            memberships.iter().any(|m| {
                memberships_of_issuer
                    .iter()
                    .any(|m2| m.group_id() == m2.group_id())
            })
        }
        ActorContext::NoLogin => false,
    }
}

/// 承認申請を作成できるか
pub fn can_create_approval_request(actor_ctx: &ActorContext) -> bool {
    matches!(actor_ctx, ActorContext::User { .. })
}

/// 承認申請を承認/却下できるか（管理者用）
pub fn can_approve_or_reject_approval_request(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:approval-request:approve".to_string())
        }
        _ => false,
    }
}

/// 承認申請をクローズできるか（申請者本人のみ）
pub fn can_close_approval_request(actor_ctx: &ActorContext, request: &ApprovalRequest) -> bool {
    match actor_ctx {
        ActorContext::User { user_id, .. } => *user_id == request.issued_by(),
        _ => false,
    }
}

/// 承認申請を削除できるか（管理者用）
pub fn can_delete_approval_request(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:approval-request:delete".to_string())
        }
        _ => false,
    }
}

pub fn can_get_document_by_id(
    actor_ctx: &ActorContext,
    document: &Document,
) -> Result<(), CanGetByIdError> {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            if claims.contains(&"koudaisai-portal:admin:document:read".to_string()) {
                Ok(())
            } else {
                Err(CanGetByIdError::Unauthorized)
            }
        }
        ActorContext::User { .. } => {
            if document
                .targets()
                .iter()
                .any(|t| t.does_actor_match(actor_ctx))
            {
                Ok(())
            } else {
                // 閲覧権限がない場合は、セキュリティのため「存在しない」として扱う
                Err(CanGetByIdError::NotFound)
            }
        }

        ActorContext::NoLogin => {
            if document
                .targets()
                .iter()
                .any(|t| t.does_actor_match(actor_ctx))
            {
                Ok(())
            } else {
                Err(CanGetByIdError::NotFound)
            }
        }
    }
}

pub fn can_get_all_document(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:document:read".to_string())
        }
        _ => true, // 後々、絞り込むときに0件になる
    }
}

pub fn can_create_document(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:document:create".to_string())
        }
        _ => false,
    }
}

pub fn can_update_document(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:document:update".to_string())
        }
        _ => false,
    }
}

pub fn can_delete_document(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:document:delete".to_string())
        }
        _ => false,
    }
}

pub fn can_get_document_category_by_id(actor_ctx: &ActorContext) -> Result<(), CanGetByIdError> {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            if claims.contains(&"koudaisai-portal:admin:document-category:read".to_string()) {
                Ok(())
            } else {
                Err(CanGetByIdError::Unauthorized)
            }
        }
        ActorContext::User { .. } => Ok(()),
        ActorContext::NoLogin => Ok(()),
    }
}

pub fn can_get_all_document_categories(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:document-category:read".to_string())
        }
        ActorContext::User { .. } => true,
        ActorContext::NoLogin => false,
    }
}

pub fn can_create_document_category(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:document-category:create".to_string())
        }
        _ => false,
    }
}

pub fn can_update_document_category(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:document-category:update".to_string())
        }
        _ => false,
    }
}

pub fn can_delete_document_category(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:document-category:delete".to_string())
        }
        _ => false,
    }
}
