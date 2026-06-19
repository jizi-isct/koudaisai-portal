use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::notification_repo::NotificationRepo;
use crate::domain::notification::Notification;
use crate::domain::notification_id::NotificationId;
use crate::domain::user_id::UserId;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

pub struct MemoryNotificationRepo {
    notifications: Arc<RwLock<HashMap<NotificationId, Notification>>>,
    /// (user_id, notification_id) が存在すれば既読。
    reads: Arc<RwLock<HashSet<(UserId, NotificationId)>>>,
}

impl Default for MemoryNotificationRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryNotificationRepo {
    pub fn new() -> Self {
        Self {
            notifications: Arc::new(RwLock::new(HashMap::new())),
            reads: Arc::new(RwLock::new(HashSet::new())),
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

    async fn find_read_ids_by_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<NotificationId>, FindError> {
        let reads = self
            .reads
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(reads
            .iter()
            .filter(|(uid, _)| *uid == user_id)
            .map(|(_, nid)| *nid)
            .collect())
    }

    async fn mark_read(
        &self,
        user_id: UserId,
        notification_id: NotificationId,
        _read_at: DateTime<Utc>,
    ) -> Result<(), InsertError> {
        let mut reads = self
            .reads
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        reads.insert((user_id, notification_id));
        Ok(())
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
        self.insert(notification)
            .await
            .map_err(|e| anyhow::anyhow!(e))
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
        self.update(notification)
            .await
            .map_err(|e| anyhow::anyhow!(e))
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
