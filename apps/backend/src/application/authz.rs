use crate::domain::actor_ctx::ActorContext;
use crate::domain::document::Document;
use crate::domain::form::Form;
use crate::domain::membership::Membership;
use crate::domain::notification::Notification;
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

pub fn can_update_user(actor_ctx: &ActorContext, _user: &User) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:user:update".to_string())
        }
        _ => false,
    }
}

pub fn can_change_m_address_of_the_user(actor_ctx: &ActorContext, _user_id: UserId) -> bool {
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

/// グループのメンバー（所属・役職）を追加・削除できるか．
// TODO: 現状は管理者(group:update クレーム)のみ許可．団体メンバー自身による管理を許す場合はここを拡張する．
pub fn can_manage_group_members(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:group:update".to_string())
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
    _request: &ApprovalRequest,
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

pub fn can_get_document(
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
        ActorContext::NoLogin => true,
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

pub fn can_get_all_notifications(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:notification:read".to_string())
        }
        _ => false,
    }
}

pub fn can_get_notification(actor_ctx: &ActorContext, notification: &Notification) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:notification:read".to_string())
        }
        ActorContext::User { .. } | ActorContext::NoLogin => notification
            .targets()
            .iter()
            .any(|target| target.does_actor_match(actor_ctx)),
    }
}

pub fn can_create_notification(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:notification:create".to_string())
        }
        _ => false,
    }
}

pub fn can_update_notification(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:notification:update".to_string())
        }
        _ => false,
    }
}

pub fn can_delete_notification(actor_ctx: &ActorContext) -> bool {
    match actor_ctx {
        ActorContext::Admin { claims, .. } => {
            claims.contains(&"koudaisai-portal:admin:notification:delete".to_string())
        }
        _ => false,
    }
}

/// ファイルのアップロード用URLを要求できるか。
pub fn can_upload_file(actor_ctx: &ActorContext) -> bool {
    matches!(
        actor_ctx,
        ActorContext::Admin { .. } | ActorContext::User { .. }
    )
}

/// ファイルのダウンロード用URLを要求できるか。
/// ダウンロードは（レガシー同様）無認証で誰でも要求できる。鍵(key)を知っていることを前提とする。
pub fn can_download_file(_actor_ctx: &ActorContext) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::clock::Clock;
    use crate::domain::actor_ctx::ActorContext;
    use crate::domain::approval_request::{ApprovalRequest, ApprovalRequestType};
    use crate::domain::approval_request_id::ApprovalRequestId;
    use crate::domain::group::GroupType;
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::{Membership, Role};
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
        Membership::new(group_id, user_id, Role::Representative, &MockClock)
    }

    fn create_pending_approval_request(issued_by: UserId) -> ApprovalRequest {
        ApprovalRequest::create(
            ApprovalRequestId::generate(),
            issued_by,
            ApprovalRequestType::EditExhibitionInfo {
                description: None,
                icon_key: None,
            },
            "Need update".to_string(),
            &MockClock,
        )
        .unwrap()
    }

    #[test]
    fn test_can_get_all_users() {
        let admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:user:read".to_string()],
        };
        assert!(can_get_all_users(&admin_ctx));

        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_get_all_users(&unauthorized_admin_ctx));

        let user_ctx = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            memberships: vec![],
            group_type: GroupType::Press,
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
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:user:read".to_string()],
        };
        assert!(can_get_user_by_id(&admin_ctx, memberships_of_the_user.clone()).is_ok());

        // Admin without permission
        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
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
            name: "テストユーザー".to_string(),
            user_id: other_user_id,
            memberships: vec![create_membership(group_id, other_user_id)],
            group_type: GroupType::Press,
        };
        assert!(can_get_user_by_id(&same_group_user_ctx, memberships_of_the_user.clone()).is_ok());

        // User in a different group
        let diff_group_id = GroupId::new('G', 2).unwrap();
        let diff_group_user_ctx = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id: other_user_id,
            memberships: vec![create_membership(diff_group_id, other_user_id)],
            group_type: GroupType::Press,
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
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:user:update".to_string()],
        };
        assert!(can_update_user(&admin_ctx, &user));

        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
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
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:user:change-email".to_string()],
        };
        assert!(can_change_m_address_of_the_user(&admin_ctx, user_id));

        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
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
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:group:read".to_string()],
        };
        assert!(can_get_all_groups(&admin_ctx));

        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
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
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:group:read".to_string()],
        };
        assert!(can_get_group_by_id(&admin_ctx, &members).is_ok());

        // Admin without permission
        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(matches!(
            can_get_group_by_id(&unauthorized_admin_ctx, &members),
            Err(CanGetByIdError::Unauthorized)
        ));

        // User who is a member
        let member_user_ctx = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id,
            memberships: vec![create_membership(group_id, user_id)],
            group_type: GroupType::Press,
        };
        assert!(can_get_group_by_id(&member_user_ctx, &members).is_ok());

        // User who is not a member
        let other_user_id = create_user_id();
        let non_member_user_ctx = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id: other_user_id,
            memberships: vec![],
            group_type: GroupType::Press,
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
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:group:create".to_string()],
        };
        assert!(can_create_group(&admin_ctx));

        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_create_group(&unauthorized_admin_ctx));

        assert!(!can_create_group(&ActorContext::NoLogin));
    }

    #[test]
    fn test_can_get_group_approval_requests() {
        let group_id = create_group_id();
        let user_id = create_user_id();

        let admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:approval-request:read".to_string()],
        };
        assert!(can_get_group_approval_requests(&admin_ctx, group_id));

        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_get_group_approval_requests(
            &unauthorized_admin_ctx,
            group_id
        ));

        let user_ctx_same_group = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id,
            memberships: vec![create_membership(group_id, user_id)],
            group_type: GroupType::Press,
        };
        assert!(can_get_group_approval_requests(
            &user_ctx_same_group,
            group_id
        ));

        let other_group = GroupId::new('G', 2).unwrap();
        let user_ctx_other_group = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id,
            memberships: vec![create_membership(other_group, user_id)],
            group_type: GroupType::Press,
        };
        assert!(!can_get_group_approval_requests(
            &user_ctx_other_group,
            group_id
        ));

        assert!(!can_get_group_approval_requests(
            &ActorContext::NoLogin,
            group_id
        ));
    }

    #[test]
    fn test_can_get_approval_request() {
        let issuer_id = create_user_id();
        let issuer_group = create_group_id();
        let memberships_of_issuer = vec![create_membership(issuer_group, issuer_id)];
        let request = create_pending_approval_request(issuer_id);

        let admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:approval-request:read".to_string()],
        };
        assert!(can_get_approval_request(
            &admin_ctx,
            &request,
            &memberships_of_issuer
        ));

        let unauthorized_admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec![],
        };
        assert!(!can_get_approval_request(
            &unauthorized_admin_ctx,
            &request,
            &memberships_of_issuer
        ));

        let viewer_id = create_user_id();
        let same_group_user_ctx = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id: viewer_id,
            memberships: vec![create_membership(issuer_group, viewer_id)],
            group_type: GroupType::Press,
        };
        assert!(can_get_approval_request(
            &same_group_user_ctx,
            &request,
            &memberships_of_issuer
        ));

        let other_group = GroupId::new('G', 2).unwrap();
        let other_group_user_ctx = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id: viewer_id,
            memberships: vec![create_membership(other_group, viewer_id)],
            group_type: GroupType::Press,
        };
        assert!(!can_get_approval_request(
            &other_group_user_ctx,
            &request,
            &memberships_of_issuer
        ));

        assert!(!can_get_approval_request(
            &ActorContext::NoLogin,
            &request,
            &memberships_of_issuer
        ));
    }

    #[test]
    fn test_can_close_approval_request() {
        let issuer_id = create_user_id();
        let request = create_pending_approval_request(issuer_id);

        let issuer_ctx = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id: issuer_id,
            memberships: vec![],
            group_type: GroupType::Press,
        };
        assert!(can_close_approval_request(&issuer_ctx, &request));

        let other_user_id = create_user_id();
        let other_user_ctx = ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id: other_user_id,
            memberships: vec![],
            group_type: GroupType::Press,
        };
        assert!(!can_close_approval_request(&other_user_ctx, &request));

        let admin_ctx = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: create_user_id(),
            claims: vec!["koudaisai-portal:admin:approval-request:approve".to_string()],
        };
        assert!(!can_close_approval_request(&admin_ctx, &request));

        assert!(!can_close_approval_request(
            &ActorContext::NoLogin,
            &request
        ));
    }
}
