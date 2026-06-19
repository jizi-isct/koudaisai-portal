use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::domain::admin_id::AdminId;
use crate::domain::approval_request::{
    ApprovalRequest, ApprovalRequestStatus, ApprovalRequestType,
};
use crate::domain::approval_request_id::ApprovalRequestId;
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt, to_insert_error};
use async_trait::async_trait;
use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;

pub struct SqliteApprovalRequestRepo {
    pool: SqlitePool,
}

impl SqliteApprovalRequestRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// `ApprovalRequestType` を type/description/icon_key 列へ分解。
struct TypeCols {
    type_: &'static str,
    description: Option<String>,
    icon_key: Option<String>,
}

fn type_cols(request: &ApprovalRequest) -> TypeCols {
    match request.request_type() {
        ApprovalRequestType::EditExhibitionInfo {
            description,
            icon_key,
        } => TypeCols {
            type_: "edit_exhibition_info",
            description: description.clone(),
            icon_key: icon_key.clone(),
        },
    }
}

/// `ApprovalRequestStatus`(代数的データ型)を各列へ分解。
struct StatusCols {
    status: &'static str,
    approved_by: Option<String>,
    approved_at: Option<i64>,
    approval_reason: Option<String>,
    rejected_by: Option<String>,
    rejected_at: Option<i64>,
    rejection_reason: Option<String>,
    closed_at: Option<i64>,
}

fn status_cols(request: &ApprovalRequest) -> StatusCols {
    let empty = StatusCols {
        status: "pending",
        approved_by: None,
        approved_at: None,
        approval_reason: None,
        rejected_by: None,
        rejected_at: None,
        rejection_reason: None,
        closed_at: None,
    };
    match request.status() {
        ApprovalRequestStatus::Pending => empty,
        ApprovalRequestStatus::Approved {
            approved_by,
            approved_at,
            approval_reason,
        } => StatusCols {
            status: "approved",
            approved_by: Some(Uuid::from(*approved_by).to_string()),
            approved_at: Some(dt_to_ms(*approved_at)),
            approval_reason: approval_reason.clone(),
            ..empty
        },
        ApprovalRequestStatus::Rejected {
            rejected_by,
            rejected_at,
            rejection_reason,
        } => StatusCols {
            status: "rejected",
            rejected_by: Some(Uuid::from(*rejected_by).to_string()),
            rejected_at: Some(dt_to_ms(*rejected_at)),
            rejection_reason: rejection_reason.clone(),
            ..empty
        },
        ApprovalRequestStatus::Closed { closed_at } => StatusCols {
            status: "closed",
            closed_at: Some(dt_to_ms(*closed_at)),
            ..empty
        },
    }
}

/// type / description / icon_key 列から `ApprovalRequestType` を復元する。
fn request_type_from_cols(
    type_: String,
    description: Option<String>,
    icon_key: Option<String>,
) -> anyhow::Result<ApprovalRequestType> {
    Ok(match type_.as_str() {
        "edit_exhibition_info" => ApprovalRequestType::EditExhibitionInfo {
            description,
            icon_key,
        },
        other => return Err(anyhow::anyhow!("unknown approval request type: {}", other)),
    })
}

/// status 列群から `ApprovalRequestStatus`(代数的データ型)を復元する。
#[allow(clippy::too_many_arguments)]
fn request_status_from_cols(
    status: String,
    approved_by: Option<String>,
    approved_at: Option<i64>,
    approval_reason: Option<String>,
    rejected_by: Option<String>,
    rejected_at: Option<i64>,
    rejection_reason: Option<String>,
    closed_at: Option<i64>,
) -> anyhow::Result<ApprovalRequestStatus> {
    Ok(match status.as_str() {
        "pending" => ApprovalRequestStatus::Pending,
        "approved" => ApprovalRequestStatus::Approved {
            approved_by: AdminId::new(Uuid::parse_str(
                &approved_by
                    .ok_or_else(|| anyhow::anyhow!("approved request missing approved_by"))?,
            )?),
            approved_at: ms_to_dt(
                approved_at
                    .ok_or_else(|| anyhow::anyhow!("approved request missing approved_at"))?,
            )?,
            approval_reason,
        },
        "rejected" => ApprovalRequestStatus::Rejected {
            rejected_by: AdminId::new(Uuid::parse_str(
                &rejected_by
                    .ok_or_else(|| anyhow::anyhow!("rejected request missing rejected_by"))?,
            )?),
            rejected_at: ms_to_dt(
                rejected_at
                    .ok_or_else(|| anyhow::anyhow!("rejected request missing rejected_at"))?,
            )?,
            rejection_reason,
        },
        "closed" => ApprovalRequestStatus::Closed {
            closed_at: ms_to_dt(
                closed_at.ok_or_else(|| anyhow::anyhow!("closed request missing closed_at"))?,
            )?,
        },
        other => {
            return Err(anyhow::anyhow!(
                "unknown approval request status: {}",
                other
            ));
        }
    })
}

async fn exec_insert<'c, E>(executor: E, request: &ApprovalRequest) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let t = type_cols(request);
    let s = status_cols(request);
    let id = request.id().as_uuid().to_string();
    let issued_at = dt_to_ms(*request.issued_at());
    let issued_by = Uuid::from(request.issued_by()).to_string();
    let issue_reason = request.issue_reason().to_string();
    sqlx::query!(
        "INSERT INTO approval_requests \
         (id, issued_at, issued_by, type, status, description, icon_key, issue_reason, \
          approved_by, approved_at, approval_reason, rejected_by, rejected_at, rejection_reason, closed_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        id,
        issued_at,
        issued_by,
        t.type_,
        s.status,
        t.description,
        t.icon_key,
        issue_reason,
        s.approved_by,
        s.approved_at,
        s.approval_reason,
        s.rejected_by,
        s.rejected_at,
        s.rejection_reason,
        s.closed_at,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_update<'c, E>(executor: E, request: &ApprovalRequest) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let t = type_cols(request);
    let s = status_cols(request);
    let id = request.id().as_uuid().to_string();
    let issued_at = dt_to_ms(*request.issued_at());
    let issued_by = Uuid::from(request.issued_by()).to_string();
    let issue_reason = request.issue_reason().to_string();
    let res = sqlx::query!(
        "UPDATE approval_requests SET \
         issued_at = ?, issued_by = ?, type = ?, status = ?, description = ?, icon_key = ?, issue_reason = ?, \
         approved_by = ?, approved_at = ?, approval_reason = ?, rejected_by = ?, rejected_at = ?, rejection_reason = ?, closed_at = ? \
         WHERE id = ?",
        issued_at,
        issued_by,
        t.type_,
        s.status,
        t.description,
        t.icon_key,
        issue_reason,
        s.approved_by,
        s.approved_at,
        s.approval_reason,
        s.rejected_by,
        s.rejected_at,
        s.rejection_reason,
        s.closed_at,
        id,
    )
    .execute(executor)
    .await?;
    Ok(res.rows_affected())
}

async fn exec_delete<'c, E>(executor: E, id: ApprovalRequestId) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = id.as_uuid().to_string();
    let res = sqlx::query!("DELETE FROM approval_requests WHERE id = ?", id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

#[async_trait]
impl ApprovalRequestRepo<SqliteTransaction> for SqliteApprovalRequestRepo {
    async fn find_by_id(
        &self,
        id: ApprovalRequestId,
    ) -> Result<Option<ApprovalRequest>, FindError> {
        let id = id.as_uuid().to_string();
        let row = sqlx::query!(
            r#"SELECT id, issued_at, issued_by, type, status, description, icon_key, issue_reason,
                      approved_by, approved_at, approval_reason, rejected_by, rejected_at, rejection_reason, closed_at
               FROM approval_requests WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<ApprovalRequest> {
            Ok(ApprovalRequest::restore(
                ApprovalRequestId::new(Uuid::parse_str(&r.id)?),
                ms_to_dt(r.issued_at)?,
                UserId::new(Uuid::parse_str(&r.issued_by)?),
                request_type_from_cols(r.r#type, r.description, r.icon_key)?,
                request_status_from_cols(
                    r.status,
                    r.approved_by,
                    r.approved_at,
                    r.approval_reason,
                    r.rejected_by,
                    r.rejected_at,
                    r.rejection_reason,
                    r.closed_at,
                )?,
                r.issue_reason,
            ))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_all(&self) -> Result<Vec<ApprovalRequest>, FindError> {
        let rows = sqlx::query!(
            r#"SELECT id, issued_at, issued_by, type, status, description, icon_key, issue_reason,
                      approved_by, approved_at, approval_reason, rejected_by, rejected_at, rejection_reason, closed_at
               FROM approval_requests"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<ApprovalRequest> {
                Ok(ApprovalRequest::restore(
                    ApprovalRequestId::new(Uuid::parse_str(&r.id)?),
                    ms_to_dt(r.issued_at)?,
                    UserId::new(Uuid::parse_str(&r.issued_by)?),
                    request_type_from_cols(r.r#type, r.description, r.icon_key)?,
                    request_status_from_cols(
                        r.status,
                        r.approved_by,
                        r.approved_at,
                        r.approval_reason,
                        r.rejected_by,
                        r.rejected_at,
                        r.rejection_reason,
                        r.closed_at,
                    )?,
                    r.issue_reason,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn find_by_user_ids(
        &self,
        user_ids: &[UserId],
    ) -> Result<Vec<ApprovalRequest>, FindError> {
        if user_ids.is_empty() {
            return Ok(Vec::new());
        }
        // 可変長 IN を静的クエリで表現するため，uuid の JSON 配列を json_each で展開する。
        let ids: Vec<String> = user_ids
            .iter()
            .map(|u| Uuid::from(*u).to_string())
            .collect();
        let ids_json =
            serde_json::to_string(&ids).map_err(|e| FindError::InternalError(e.into()))?;
        let rows = sqlx::query!(
            r#"SELECT id, issued_at, issued_by, type, status, description, icon_key, issue_reason,
                      approved_by, approved_at, approval_reason, rejected_by, rejected_at, rejection_reason, closed_at
               FROM approval_requests
               WHERE issued_by IN (SELECT value FROM json_each(?))"#,
            ids_json,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<ApprovalRequest> {
                Ok(ApprovalRequest::restore(
                    ApprovalRequestId::new(Uuid::parse_str(&r.id)?),
                    ms_to_dt(r.issued_at)?,
                    UserId::new(Uuid::parse_str(&r.issued_by)?),
                    request_type_from_cols(r.r#type, r.description, r.icon_key)?,
                    request_status_from_cols(
                        r.status,
                        r.approved_by,
                        r.approved_at,
                        r.approval_reason,
                        r.rejected_by,
                        r.rejected_at,
                        r.rejection_reason,
                        r.closed_at,
                    )?,
                    r.issue_reason,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, request: &ApprovalRequest) -> Result<(), InsertError> {
        exec_insert(&self.pool, request)
            .await
            .map_err(to_insert_error)
    }

    async fn insert_in(
        &self,
        tx: &mut SqliteTransaction,
        request: &ApprovalRequest,
    ) -> Result<(), anyhow::Error> {
        exec_insert(tx.conn()?, request).await.map_err(Into::into)
    }

    async fn update(&self, request: &ApprovalRequest) -> Result<(), UpdateError> {
        let affected = exec_update(&self.pool, request)
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
        request: &ApprovalRequest,
    ) -> Result<(), anyhow::Error> {
        let affected = exec_update(tx.conn()?, request).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!(
                "approval request not found: {}",
                request.id()
            ));
        }
        Ok(())
    }

    async fn delete(&self, id: ApprovalRequestId) -> Result<(), DeleteError> {
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
        id: ApprovalRequestId,
    ) -> Result<(), anyhow::Error> {
        let affected = exec_delete(tx.conn()?, id).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!("approval request not found: {}", id));
        }
        Ok(())
    }
}
