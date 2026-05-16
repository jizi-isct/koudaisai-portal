use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::notification_repo::NotificationRepo;
use crate::domain::notification::Notification;
use crate::domain::notification_id::NotificationId;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct MemoryNotificationRepo {
    notifications: Arc<RwLock<HashMap<NotificationId, Notification>>>,
}

impl MemoryNotificationRepo {
    pub fn new() -> Self {
        Self {
            notifications: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl NotificationRepo<MemoryTransaction> for MemoryNotificationRepo {
    async fn find_by_id(&self, id: NotificationId) -> Result<Option<Notification>, FindError> {
        let notifications = self
            .notifications
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(notifications.get(&id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Notification>, FindError> {
        let notifications = self
            .notifications
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(notifications.values().cloned().collect())
    }

    async fn insert(&self, notification: &Notification) -> Result<(), InsertError> {
        let mut notifications = self
            .notifications
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if notifications.contains_key(&notification.id()) {
            return Err(InsertError::Conflict);
        }
        notifications.insert(notification.id(), notification.clone());
        Ok(())
    }

    async fn insert_in(
        &self,
        _tx: &mut MemoryTransaction,
        notification: &Notification,
    ) -> Result<(), anyhow::Error> {
        self.insert(notification).await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn update(&self, notification: &Notification) -> Result<(), UpdateError> {
        let mut notifications = self
            .notifications
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if !notifications.contains_key(&notification.id()) {
            return Err(UpdateError::NotFound);
        }
        notifications.insert(notification.id(), notification.clone());
        Ok(())
    }

    async fn update_in(
        &self,
        _tx: &mut MemoryTransaction,
        notification: &Notification,
    ) -> Result<(), anyhow::Error> {
        self.update(notification).await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn delete(&self, id: NotificationId) -> Result<(), DeleteError> {
        let mut notifications = self
            .notifications
            .write()
            .map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        if notifications.remove(&id).is_none() {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(
        &self,
        _tx: &mut MemoryTransaction,
        id: NotificationId,
    ) -> Result<(), anyhow::Error> {
        self.delete(id).await.map_err(|e| anyhow::anyhow!(e))
    }
}
