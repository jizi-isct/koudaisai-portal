use crate::application::authz;
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::application::ports::clock::Clock;
use crate::application::ports::repositories::notification_repo::NotificationRepo;
use crate::application::transaction::Transaction;
use crate::domain::actor_ctx::ActorContext;
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
        target: Vec<TargetSpecifier>,
        notification_type: NotificationType,
    ) -> Result<NotificationId, ApplicationOperationError<InsertError>> {
        if !authz::can_create_notification(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let created_by = match actor_ctx {
            ActorContext::Admin { user_id, .. } => Some(*user_id),
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let id = NotificationId::generate();
        let notification = Notification::create(id, target, notification_type, created_by, self.clock)
            .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;

        self.notification_repo.insert(&notification).await?;
        Ok(id)
    }

    pub async fn update(
        &self,
        actor_ctx: &ActorContext,
        id: NotificationId,
        target: Option<Vec<TargetSpecifier>>,
        markdown: Option<(String, String)>,
    ) -> Result<Notification, ApplicationOperationError<UpdateError>> {
        if !authz::can_update_notification(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let updated_by = match actor_ctx {
            ActorContext::Admin { user_id, .. } => Some(*user_id),
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let mut notification = self
            .notification_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
            .ok_or(ApplicationOperationError::OperationFailed(UpdateError::NotFound))?;

        if let Some(target) = target {
            notification
                .update_target(target, updated_by, self.clock)
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