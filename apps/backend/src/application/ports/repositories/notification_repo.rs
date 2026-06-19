use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::transaction::Transaction;
use crate::domain::notification::Notification;
use crate::domain::notification_id::NotificationId;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};

#[async_trait::async_trait]
pub trait NotificationRepo<Tx: Transaction> {
    async fn find_by_id(&self, id: NotificationId) -> Result<Option<Notification>, FindError>;
    async fn find_all(&self) -> Result<Vec<Notification>, FindError>;

    /// 指定ユーザーが既読にした通知 ID 一覧。`GET /users/{id}/notifications` の
    /// is_read 算出に使う。行が存在する通知のみ既読。
    async fn find_read_ids_by_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<NotificationId>, FindError>;

    /// 通知を既読にする(冪等な upsert)。read マーク用エンドポイント追加時に使う。
    async fn mark_read(
        &self,
        user_id: UserId,
        notification_id: NotificationId,
        read_at: DateTime<Utc>,
    ) -> Result<(), InsertError>;

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
