use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::domain::document_category::DocumentCategory;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt, to_insert_error};
use async_trait::async_trait;
use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;

pub struct SqliteDocumentCategoryRepo {
    pool: SqlitePool,
}

impl SqliteDocumentCategoryRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

async fn exec_insert<'c, E>(executor: E, c: &DocumentCategory) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = c.id().to_string();
    let created_at = dt_to_ms(c.created_at());
    let updated_at = dt_to_ms(c.updated_at());
    let title = c.title().to_string();
    let emoji = c.emoji().map(|s| s.to_string());
    sqlx::query!(
        "INSERT INTO document_categories (id, created_at, updated_at, title, emoji) \
         VALUES (?, ?, ?, ?, ?)",
        id,
        created_at,
        updated_at,
        title,
        emoji,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_update<'c, E>(executor: E, c: &DocumentCategory) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = c.id().to_string();
    let created_at = dt_to_ms(c.created_at());
    let updated_at = dt_to_ms(c.updated_at());
    let title = c.title().to_string();
    let emoji = c.emoji().map(|s| s.to_string());
    let res = sqlx::query!(
        "UPDATE document_categories SET created_at = ?, updated_at = ?, title = ?, emoji = ? \
         WHERE id = ?",
        created_at,
        updated_at,
        title,
        emoji,
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
    let res = sqlx::query!("DELETE FROM document_categories WHERE id = ?", id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

#[async_trait]
impl DocumentCategoryRepo<SqliteTransaction> for SqliteDocumentCategoryRepo {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DocumentCategory>, FindError> {
        let id = id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, created_at, updated_at, title, emoji
               FROM document_categories WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<DocumentCategory> {
            Ok(DocumentCategory::restore(
                Uuid::parse_str(&r.id)?,
                ms_to_dt(r.created_at)?,
                ms_to_dt(r.updated_at)?,
                r.title,
                r.emoji,
            ))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_all(&self) -> Result<Vec<DocumentCategory>, FindError> {
        let rows = sqlx::query!(
            r#"SELECT id, created_at, updated_at, title, emoji
               FROM document_categories"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<DocumentCategory> {
                Ok(DocumentCategory::restore(
                    Uuid::parse_str(&r.id)?,
                    ms_to_dt(r.created_at)?,
                    ms_to_dt(r.updated_at)?,
                    r.title,
                    r.emoji,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, document_category: &DocumentCategory) -> Result<(), InsertError> {
        exec_insert(&self.pool, document_category)
            .await
            .map_err(to_insert_error)
    }

    async fn insert_in(
        &self,
        tx: &mut SqliteTransaction,
        document_category: &DocumentCategory,
    ) -> Result<(), InsertError> {
        let conn = tx.conn().map_err(InsertError::InternalError)?;
        exec_insert(conn, document_category)
            .await
            .map_err(to_insert_error)
    }

    async fn update(&self, document_category: &DocumentCategory) -> Result<(), UpdateError> {
        let affected = exec_update(&self.pool, document_category)
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
        document_category: &DocumentCategory,
    ) -> Result<(), UpdateError> {
        let conn = tx.conn().map_err(UpdateError::InternalError)?;
        let affected = exec_update(conn, document_category)
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
