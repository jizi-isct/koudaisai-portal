
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

    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<Notification>, ApplicationOperationError<FindError>> {
        if !authz::can_get_all_notifications(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        Ok(self.notification_repo.find_all().await?)
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
