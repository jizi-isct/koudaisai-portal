use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::transaction::Transaction;
use crate::domain::notification::Notification;
use crate::domain::notification_id::NotificationId;

#[async_trait::async_trait]
pub trait NotificationRepo<Tx: Transaction> {
    async fn find_by_id(&self, id: NotificationId) -> Result<Option<Notification>, FindError>;
    async fn find_all(&self) -> Result<Vec<Notification>, FindError>;
    async fn insert(&self, notification: &Notification) -> Result<(), InsertError>;
    async fn insert_in(
        &self,
        tx: &mut Tx,
        notification: &Notification,
    ) -> Result<(), anyhow::Error>;
    async fn update(&self, notification: &Notification) -> Result<(), UpdateError>;
    async fn update_in(
        &self,
        tx: &mut Tx,
        notification: &Notification,
    ) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: NotificationId) -> Result<(), DeleteError>;
    async fn delete_in(&self, tx: &mut Tx, id: NotificationId) -> Result<(), anyhow::Error>;
}
