use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::document_repo::DocumentRepo;
use crate::domain::admin_id::AdminId;
use crate::domain::document::{Document, DocumentFormat};
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt, targets_from_json, targets_to_json, to_insert_error};
use async_trait::async_trait;
use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;

pub struct SqliteDocumentRepo {
    pool: SqlitePool,
}

impl SqliteDocumentRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// `DocumentFormat`(代数的データ型)をテーブルの各列へ分解した中間表現。
struct FormatCols {
    format: &'static str,
    content: Option<String>,
    file_key: Option<String>,
    file_name: Option<String>,
}

fn format_cols(document: &Document) -> FormatCols {
    match document.format() {
        DocumentFormat::Markdown { content } => FormatCols {
            format: "markdown",
            content: Some(content.clone()),
            file_key: None,
            file_name: None,
        },
        DocumentFormat::Pdf {
            file_key,
            file_name,
        } => FormatCols {
            format: "pdf",
            content: None,
            file_key: Some(file_key.clone()),
            file_name: Some(file_name.clone()),
        },
        DocumentFormat::Misc {
            file_key,
            file_name,
        } => FormatCols {
            format: "misc",
            content: None,
            file_key: Some(file_key.clone()),
            file_name: Some(file_name.clone()),
        },
    }
}

/// format / content / file_* 列群から `DocumentFormat`(代数的データ型)を復元する。
fn document_format_from_cols(
    format: String,
    content: Option<String>,
    file_key: Option<String>,
    file_name: Option<String>,
) -> anyhow::Result<DocumentFormat> {
    Ok(match format.as_str() {
        "markdown" => DocumentFormat::Markdown {
            content: content.ok_or_else(|| anyhow::anyhow!("markdown document missing content"))?,
        },
        "pdf" => DocumentFormat::Pdf {
            file_key: file_key.ok_or_else(|| anyhow::anyhow!("pdf document missing file_key"))?,
            file_name: file_name.ok_or_else(|| anyhow::anyhow!("pdf document missing file_name"))?,
        },
        "misc" => DocumentFormat::Misc {
            file_key: file_key.ok_or_else(|| anyhow::anyhow!("misc document missing file_key"))?,
            file_name: file_name.ok_or_else(|| anyhow::anyhow!("misc document missing file_name"))?,
        },
        other => return Err(anyhow::anyhow!("unknown document format: {}", other)),
    })
}

async fn exec_insert<'c, E>(executor: E, document: &Document) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let f = format_cols(document);
    let id = document.id().to_string();
    let created_at = dt_to_ms(document.created_at());
    let updated_at = dt_to_ms(document.updated_at());
    let created_by = Uuid::from(document.created_by()).to_string();
    let updated_by = Uuid::from(document.updated_by()).to_string();
    let title = document.title().to_string();
    let category = document.category().map(|c| c.to_string());
    let targets = targets_to_json(document.targets());
    sqlx::query!(
        "INSERT INTO documents \
         (id, created_at, updated_at, created_by, updated_by, title, category, targets, format, content, file_key, file_name) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        id,
        created_at,
        updated_at,
        created_by,
        updated_by,
        title,
        category,
        targets,
        f.format,
        f.content,
        f.file_key,
        f.file_name,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_update<'c, E>(executor: E, document: &Document) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let f = format_cols(document);
    let id = document.id().to_string();
    let created_at = dt_to_ms(document.created_at());
    let updated_at = dt_to_ms(document.updated_at());
    let created_by = Uuid::from(document.created_by()).to_string();
    let updated_by = Uuid::from(document.updated_by()).to_string();
    let title = document.title().to_string();
    let category = document.category().map(|c| c.to_string());
    let targets = targets_to_json(document.targets());
    let res = sqlx::query!(
        "UPDATE documents SET \
         created_at = ?, updated_at = ?, created_by = ?, updated_by = ?, title = ?, category = ?, \
         targets = ?, format = ?, content = ?, file_key = ?, file_name = ? \
         WHERE id = ?",
        created_at,
        updated_at,
        created_by,
        updated_by,
        title,
        category,
        targets,
        f.format,
        f.content,
        f.file_key,
        f.file_name,
        id,
    )
    .execute(executor)
    .await?;
    Ok(res.rows_affected())
}

async fn exec_delete<'c, E>(executor: E, id: Uuid) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = id.to_string();
    let res = sqlx::query!("DELETE FROM documents WHERE id = ?", id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

#[async_trait]
impl DocumentRepo<SqliteTransaction> for SqliteDocumentRepo {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Document>, FindError> {
        let id = id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, created_at, updated_at, created_by, updated_by, title,
                      category, targets, format, content, file_key, file_name
               FROM documents WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<Document> {
            let category = match r.category {
                Some(c) => Some(Uuid::parse_str(&c)?),
                None => None,
            };
            Document::restore(
                Uuid::parse_str(&r.id)?,
                ms_to_dt(r.created_at)?,
                ms_to_dt(r.updated_at)?,
                AdminId::new(Uuid::parse_str(&r.created_by)?),
                AdminId::new(Uuid::parse_str(&r.updated_by)?),
                r.title,
                category,
                document_format_from_cols(r.format, r.content, r.file_key, r.file_name)?,
                targets_from_json(&r.targets)?,
            )
            .map_err(|e| anyhow::anyhow!(e.to_string()))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_all(&self) -> Result<Vec<Document>, FindError> {
        let rows = sqlx::query!(
            r#"SELECT id, created_at, updated_at, created_by, updated_by, title,
                      category, targets, format, content, file_key, file_name
               FROM documents"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Document> {
                let category = match r.category {
                    Some(c) => Some(Uuid::parse_str(&c)?),
                    None => None,
                };
                Document::restore(
                    Uuid::parse_str(&r.id)?,
                    ms_to_dt(r.created_at)?,
                    ms_to_dt(r.updated_at)?,
                    AdminId::new(Uuid::parse_str(&r.created_by)?),
                    AdminId::new(Uuid::parse_str(&r.updated_by)?),
                    r.title,
                    category,
                    document_format_from_cols(r.format, r.content, r.file_key, r.file_name)?,
                    targets_from_json(&r.targets)?,
                )
                .map_err(|e| anyhow::anyhow!(e.to_string()))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, document: &Document) -> Result<(), InsertError> {
        exec_insert(&self.pool, document).await.map_err(to_insert_error)
    }

    async fn insert_in(
        &self,
        tx: &mut SqliteTransaction,
        document: &Document,
    ) -> Result<(), InsertError> {
        let conn = tx.conn().map_err(InsertError::InternalError)?;
        exec_insert(conn, document).await.map_err(to_insert_error)
    }

    async fn update(&self, document: &Document) -> Result<(), UpdateError> {
        let affected = exec_update(&self.pool, document)
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
        document: &Document,
    ) -> Result<(), UpdateError> {
        let conn = tx.conn().map_err(UpdateError::InternalError)?;
        let affected = exec_update(conn, document)
            .await
            .map_err(|e| UpdateError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DeleteError> {
        let affected = exec_delete(&self.pool, id)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(&self, tx: &mut SqliteTransaction, id: Uuid) -> Result<(), DeleteError> {
        let conn = tx.conn().map_err(DeleteError::InternalError)?;
        let affected = exec_delete(conn, id)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }
}
