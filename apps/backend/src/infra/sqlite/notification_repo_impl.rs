use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::notification_repo::NotificationRepo;
use crate::domain::admin_id::AdminId;
use crate::domain::approval_request_id::ApprovalRequestId;
use crate::domain::notification::{Notification, NotificationType};
use crate::domain::notification_id::NotificationId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt, targets_from_json, targets_to_json, to_insert_error};
use async_trait::async_trait;
use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;

pub struct SqliteNotificationRepo {
    pool: SqlitePool,
}

impl SqliteNotificationRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// `NotificationType`(代数的データ型)を各列へ分解。
struct TypeCols {
    type_: &'static str,
    approval_request_id: Option<String>,
    title: Option<String>,
    content: Option<String>,
}

fn type_cols(notification: &Notification) -> TypeCols {
    match notification.notification_type() {
        NotificationType::Markdown { title, content } => TypeCols {
            type_: "markdown",
            approval_request_id: None,
            title: Some(title.clone()),
            content: Some(content.clone()),
        },
        NotificationType::ApprovalRequest {
            approval_request_id,
        } => TypeCols {
            type_: "approval_request",
            approval_request_id: Some(approval_request_id.as_uuid().to_string()),
            title: None,
            content: None,
        },
    }
}

fn opt_admin_id(v: Option<String>) -> anyhow::Result<Option<AdminId>> {
    match v {
        Some(s) => Ok(Some(AdminId::new(Uuid::parse_str(&s)?))),
        None => Ok(None),
    }
}

/// type / approval_request_id / title / content 列群から `NotificationType` を復元する。
fn notification_type_from_cols(
    type_: String,
    approval_request_id: Option<String>,
    title: Option<String>,
    content: Option<String>,
) -> anyhow::Result<NotificationType> {
    Ok(match type_.as_str() {
        "markdown" => NotificationType::Markdown {
            title: title.ok_or_else(|| anyhow::anyhow!("markdown notification missing title"))?,
            content: content
                .ok_or_else(|| anyhow::anyhow!("markdown notification missing content"))?,
        },
        "approval_request" => NotificationType::ApprovalRequest {
            approval_request_id: ApprovalRequestId::new(Uuid::parse_str(
                &approval_request_id.ok_or_else(|| {
                    anyhow::anyhow!("approval notification missing approval_request_id")
                })?,
            )?),
        },
        other => return Err(anyhow::anyhow!("unknown notification type: {}", other)),
    })
}

async fn exec_insert<'c, E>(executor: E, notification: &Notification) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let t = type_cols(notification);
    let id = notification.id().as_uuid().to_string();
    let created_at = dt_to_ms(*notification.created_at());
    let updated_at = dt_to_ms(*notification.updated_at());
    let created_by = notification.created_by().map(|u| Uuid::from(u).to_string());
    let updated_by = notification.updated_by().map(|u| Uuid::from(u).to_string());
    let targets = targets_to_json(notification.targets());
    sqlx::query!(
        "INSERT INTO notifications \
         (id, created_at, updated_at, created_by, updated_by, targets, type, approval_request_id, title, content) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        id,
        created_at,
        updated_at,
        created_by,
        updated_by,
        targets,
        t.type_,
        t.approval_request_id,
        t.title,
        t.content,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_update<'c, E>(executor: E, notification: &Notification) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let t = type_cols(notification);
    let id = notification.id().as_uuid().to_string();
    let created_at = dt_to_ms(*notification.created_at());
    let updated_at = dt_to_ms(*notification.updated_at());
    let created_by = notification.created_by().map(|u| Uuid::from(u).to_string());
    let updated_by = notification.updated_by().map(|u| Uuid::from(u).to_string());
    let targets = targets_to_json(notification.targets());
    let res = sqlx::query!(
        "UPDATE notifications SET \
         created_at = ?, updated_at = ?, created_by = ?, updated_by = ?, targets = ?, \
         type = ?, approval_request_id = ?, title = ?, content = ? \
         WHERE id = ?",
        created_at,
        updated_at,
        created_by,
        updated_by,
        targets,
        t.type_,
        t.approval_request_id,
        t.title,
        t.content,
        id,
    )
    .execute(executor)
    .await?;
    Ok(res.rows_affected())
}

async fn exec_delete<'c, E>(executor: E, id: NotificationId) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = id.as_uuid().to_string();
    let res = sqlx::query!("DELETE FROM notifications WHERE id = ?", id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

#[async_trait]
impl NotificationRepo<SqliteTransaction> for SqliteNotificationRepo {
    async fn find_by_id(&self, id: NotificationId) -> Result<Option<Notification>, FindError> {
        let id = id.as_uuid().to_string();
        let row = sqlx::query!(
            r#"SELECT id, created_at, updated_at, created_by, updated_by, targets,
                      type, approval_request_id, title, content
               FROM notifications WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<Notification> {
            Notification::restore(
                NotificationId::new(Uuid::parse_str(&r.id)?),
                ms_to_dt(r.created_at)?,
                ms_to_dt(r.updated_at)?,
                opt_admin_id(r.created_by)?,
                opt_admin_id(r.updated_by)?,
                targets_from_json(&r.targets)?,
                notification_type_from_cols(r.r#type, r.approval_request_id, r.title, r.content)?,
            )
            .map_err(|e| anyhow::anyhow!(e.to_string()))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_all(&self) -> Result<Vec<Notification>, FindError> {
        let rows = sqlx::query!(
            r#"SELECT id, created_at, updated_at, created_by, updated_by, targets,
                      type, approval_request_id, title, content
               FROM notifications"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Notification> {
                Notification::restore(
                    NotificationId::new(Uuid::parse_str(&r.id)?),
                    ms_to_dt(r.created_at)?,
                    ms_to_dt(r.updated_at)?,
                    opt_admin_id(r.created_by)?,
                    opt_admin_id(r.updated_by)?,
                    targets_from_json(&r.targets)?,
                    notification_type_from_cols(r.r#type, r.approval_request_id, r.title, r.content)?,
                )
                .map_err(|e| anyhow::anyhow!(e.to_string()))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, notification: &Notification) -> Result<(), InsertError> {
        exec_insert(&self.pool, notification)
            .await
            .map_err(to_insert_error)
    }

    async fn insert_in(
        &self,
        tx: &mut SqliteTransaction,
        notification: &Notification,
    ) -> Result<(), anyhow::Error> {
        exec_insert(tx.conn()?, notification).await.map_err(Into::into)
    }

    async fn update(&self, notification: &Notification) -> Result<(), UpdateError> {
        let affected = exec_update(&self.pool, notification)
            .await
            .map_err(|e| UpdateError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn update_in(
        &self,
        tx: &mut SqliteTransaction,
        notification: &Notification,
    ) -> Result<(), anyhow::Error> {
        let affected = exec_update(tx.conn()?, notification).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!("notification not found: {}", notification.id()));
        }
        Ok(())
    }

    async fn delete(&self, id: NotificationId) -> Result<(), DeleteError> {
        let affected = exec_delete(&self.pool, id)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(
        &self,
        tx: &mut SqliteTransaction,
        id: NotificationId,
    ) -> Result<(), anyhow::Error> {
        let affected = exec_delete(tx.conn()?, id).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!("notification not found: {}", id));
        }
        Ok(())
    }
}
