use crate::application::authz;
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::application::ports::clock::Clock;
use crate::application::ports::repositories::notification_repo::NotificationRepo;
use crate::application::transaction::Transaction;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::admin_id::AdminId;
use crate::domain::notification::{Notification, NotificationType};
use crate::domain::notification_id::NotificationId;
use crate::domain::target_specifier::TargetSpecifier;
use crate::domain::user_id::UserId;
use std::collections::HashSet;
use std::marker::PhantomData;

pub struct NotificationApp<'a, Tx: Transaction, NR: NotificationRepo<Tx>, C: Clock> {
    _phantom: PhantomData<&'a Tx>,
    notification_repo: &'a NR,
    clock: &'a C,
}

impl<'a, Tx: Transaction, NR: NotificationRepo<Tx>, C: Clock> NotificationApp<'a, Tx, NR, C> {
    pub fn new(notification_repo: &'a NR, clock: &'a C) -> Self {
        Self {
            _phantom: PhantomData::default(),
            notification_repo,
            clock,
        }
    }

    /// アクターが閲覧可能な Notification をすべて返す。
    /// 管理者(notification:read クレーム)は全件、参加団体/未ログインは
    /// 自身が対象(targets)に含まれる通知のみを取得する（判定は `get_by_id` と同じ
    /// `authz::can_get_notification`）。
    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<Notification>, ApplicationOperationError<FindError>> {
        let notifications = self.notification_repo.find_all().await?;
        Ok(notifications
            .into_iter()
            .filter(|notification| authz::can_get_notification(actor_ctx, notification))
            .collect())
    }

    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        id: NotificationId,
    ) -> Result<Option<Notification>, ApplicationOperationError<FindError>> {
        let Some(notification) = self.notification_repo.find_by_id(id).await? else {
            return Ok(None);
        };

        if !authz::can_get_notification(actor_ctx, &notification) {
            return Ok(None);
        }

        Ok(Some(notification))
    }

    pub async fn create(
        &self,
        actor_ctx: &ActorContext,
        targets: Vec<TargetSpecifier>,
        notification_type: NotificationType,
    ) -> Result<NotificationId, ApplicationOperationError<InsertError>> {
        if !authz::can_create_notification(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let created_by = match actor_ctx {
            ActorContext::Admin { user_id, .. } => Some(AdminId::new((*user_id).into())),
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let id = NotificationId::generate();
        let notification =
            Notification::create(id, targets, notification_type, created_by, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;

        self.notification_repo.insert(&notification).await?;
        Ok(id)
    }

    pub async fn update(
        &self,
        actor_ctx: &ActorContext,
        id: NotificationId,
        targets: Option<Vec<TargetSpecifier>>,
        markdown: Option<(String, String)>,
    ) -> Result<Notification, ApplicationOperationError<UpdateError>> {
        if !authz::can_update_notification(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let updated_by = match actor_ctx {
            ActorContext::Admin { user_id, .. } => Some(AdminId::new((*user_id).into())),
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let mut notification = self
            .notification_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
            .ok_or(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ))?;

        if let Some(targets) = targets {
            notification
                .update_target(targets, updated_by, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        }

        if let Some((title, content)) = markdown {
            notification
                .update_markdown(title, content, updated_by, self.clock)
                .map_err(|_| {
                    ApplicationOperationError::InvalidInput(
                        "Cannot update markdown for non-markdown notification".to_string(),
                    )
                })?;
        }

        self.notification_repo.update(&notification).await?;
        Ok(notification)
    }

    pub async fn delete(
        &self,
        actor_ctx: &ActorContext,
        id: NotificationId,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        if !authz::can_delete_notification(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        self.notification_repo.delete(id).await?;
        Ok(())
    }

    /// 対象ユーザー(`target_user_id`)宛ての通知を、そのユーザーの既読状態付きで返す
    /// (`GET /users/{id}/notifications`)。新しい順にソートする。
    ///
    /// `target_ctx` は対象ユーザーの認可コンテキスト(`build_actor_context` の結果)。
    /// 所属グループが無い等で構築できない場合は `None` を渡す(その場合は
    /// 自分宛て(`UserId`)と全員宛て(`UserNologin`)のみが対象)。
    /// 閲覧権限(caller)は管理者(notification:read)または本人のみ。
    pub async fn get_for_user(
        &self,
        caller_ctx: &ActorContext,
        target_user_id: UserId,
        target_ctx: Option<&ActorContext>,
    ) -> Result<Vec<(Notification, bool)>, ApplicationOperationError<FindError>> {
        if !authz::can_get_user_notifications(caller_ctx, target_user_id) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let read_ids: HashSet<NotificationId> = self
            .notification_repo
            .find_read_ids_by_user(target_user_id)
            .await?
            .into_iter()
            .collect();

        let mut notifications: Vec<Notification> = self
            .notification_repo
            .find_all()
            .await?
            .into_iter()
            .filter(|n| {
                n.targets()
                    .iter()
                    .any(|t| target_matches(t, target_user_id, target_ctx))
            })
            .collect();
        // 新しい順(created_at 降順)。
        notifications.sort_by(|a, b| b.created_at().cmp(a.created_at()));

        Ok(notifications
            .into_iter()
            .map(|n| {
                let is_read = read_ids.contains(&n.id());
                (n, is_read)
            })
            .collect())
    }
}

/// 通知のターゲットが対象ユーザーにマッチするか。
/// `target_ctx` があればドメインの照合(`does_actor_match`)を用い、無ければ
/// 自分宛て(`UserId`)・全員宛て(`UserNologin`)のみマッチとみなす。
fn target_matches(
    target: &TargetSpecifier,
    target_user_id: UserId,
    target_ctx: Option<&ActorContext>,
) -> bool {
    match target_ctx {
        Some(ctx) => target.does_actor_match(ctx),
        None => matches!(
            target,
            TargetSpecifier::UserNologin
        ) || matches!(target, TargetSpecifier::UserId(u) if *u == target_user_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::actor_ctx::ActorContext;
    use crate::domain::user_id::UserId;
    use crate::infra::memory::clock_impl::MemoryClock;
    use crate::infra::memory::notification_repo_impl::MemoryNotificationRepo;
    use chrono::Utc;
    use uuid::Uuid;

    fn admin_ctx() -> ActorContext {
        ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec![
                "koudaisai-portal:admin:notification:read".to_string(),
                "koudaisai-portal:admin:notification:create".to_string(),
                "koudaisai-portal:admin:notification:update".to_string(),
                "koudaisai-portal:admin:notification:delete".to_string(),
            ],
        }
    }

    #[tokio::test]
    async fn test_create_update_delete_notification_flow() {
        let repo = MemoryNotificationRepo::new();
        let clock = MemoryClock::new(Utc::now());
        let app = NotificationApp::new(&repo, &clock);
        let ctx = admin_ctx();

        let targets = vec![TargetSpecifier::UserNologin];
        let nt = NotificationType::markdown("title".to_string(), "body".to_string()).unwrap();
        let id = app.create(&ctx, targets.clone(), nt.clone()).await.unwrap();

        let fetched = repo.find_by_id(id).await.unwrap().unwrap();
        assert_eq!(fetched.id(), id);

        let got = app.get_by_id(&ctx, id).await.unwrap().unwrap();
        assert_eq!(got.id(), id);

        let updated = app
            .update(
                &ctx,
                id,
                Some(vec![]),
                Some(("newtitle".to_string(), "newbody".to_string())),
            )
            .await
            .unwrap();

        assert_eq!(updated.targets(), &[]);
        assert_eq!(
            updated.notification_type(),
            &NotificationType::Markdown {
                title: "newtitle".to_string(),
                content: "newbody".to_string(),
            }
        );

        app.delete(&ctx, id).await.unwrap();
        assert!(repo.find_by_id(id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_all_filters_by_target() {
        use crate::domain::group::GroupType;

        let repo = MemoryNotificationRepo::new();
        let clock = MemoryClock::new(Utc::now());
        let app = NotificationApp::new(&repo, &clock);
        let admin = admin_ctx();

        // A: 全員向け(UserNologin は誰にでもマッチ)
        let a = app
            .create(
                &admin,
                vec![TargetSpecifier::UserNologin],
                NotificationType::markdown("a".to_string(), "a".to_string()).unwrap(),
            )
            .await
            .unwrap();
        // B: 特定ユーザー向け
        let target_user = UserId::new(Uuid::new_v4());
        app.create(
            &admin,
            vec![TargetSpecifier::UserId(target_user)],
            NotificationType::markdown("b".to_string(), "b".to_string()).unwrap(),
        )
        .await
        .unwrap();

        let user_ctx = |user_id| ActorContext::User {
            name: "テストユーザー".to_string(),
            user_id,
            memberships: vec![],
            group_type: GroupType::Press,
        };

        // 管理者(read クレームあり): 全件
        assert_eq!(app.get_all(&admin).await.unwrap().len(), 2);

        // 未ログイン: 全員向けの A のみ
        let nologin = app.get_all(&ActorContext::NoLogin).await.unwrap();
        assert_eq!(nologin.len(), 1);
        assert_eq!(nologin[0].id(), a);

        // 対象ユーザー本人: A + B
        assert_eq!(app.get_all(&user_ctx(target_user)).await.unwrap().len(), 2);

        // 別ユーザー: A のみ
        let other = app
            .get_all(&user_ctx(UserId::new(Uuid::new_v4())))
            .await
            .unwrap();
        assert_eq!(other.len(), 1);
        assert_eq!(other[0].id(), a);

        // 管理者(read クレームなし): 0 件
        let admin_noclaim = ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec![],
        };
        assert!(app.get_all(&admin_noclaim).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_create_unauthorized_non_admin() {
        let repo = MemoryNotificationRepo::new();
        let clock = MemoryClock::new(Utc::now());
        let app = NotificationApp::new(&repo, &clock);
        let ctx = ActorContext::NoLogin;
        let targets = vec![TargetSpecifier::UserNologin];
        let nt = NotificationType::markdown("t".to_string(), "b".to_string()).unwrap();
        let res = app.create(&ctx, targets, nt).await;
        assert!(matches!(res, Err(ApplicationOperationError::Unauthorized)));
    }
}
