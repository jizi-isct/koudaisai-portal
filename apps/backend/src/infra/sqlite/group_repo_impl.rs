use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::domain::group::{Group, GroupType};
use crate::domain::group_id::GroupId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt, to_insert_error};
use async_trait::async_trait;
use sqlx::{Sqlite, SqlitePool};
use std::str::FromStr;

pub struct SqliteGroupRepo {
    pool: SqlitePool,
}

impl SqliteGroupRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// `GroupType` を DB に保存する snake_case のタグ文字列へ変換する。
fn type_tag(t: GroupType) -> &'static str {
    match t {
        GroupType::Press => "press",
        GroupType::GeneralProject => "general_project",
        GroupType::BoothProject => "booth_project",
        GroupType::LabProject => "lab_project",
        GroupType::StageProject => "stage_project",
    }
}

/// DB のタグ文字列から `GroupType` を復元する。
fn group_type_from_tag(s: &str) -> anyhow::Result<GroupType> {
    Ok(match s {
        "press" => GroupType::Press,
        "general_project" => GroupType::GeneralProject,
        "booth_project" => GroupType::BoothProject,
        "lab_project" => GroupType::LabProject,
        "stage_project" => GroupType::StageProject,
        other => return Err(anyhow::anyhow!("unknown group type: {}", other)),
    })
}

async fn exec_insert<'c, E>(executor: E, group: &Group) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = group.id().to_string();
    let created_at = dt_to_ms(group.created_at());
    let updated_at = dt_to_ms(group.updated_at());
    let name = group.name().to_string();
    let type_ = type_tag(*group.r#type());
    sqlx::query!(
        "INSERT INTO groups (id, created_at, updated_at, name, type) VALUES (?, ?, ?, ?, ?)",
        id,
        created_at,
        updated_at,
        name,
        type_,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_update<'c, E>(executor: E, group: &Group) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = group.id().to_string();
    let created_at = dt_to_ms(group.created_at());
    let updated_at = dt_to_ms(group.updated_at());
    let name = group.name().to_string();
    let type_ = type_tag(*group.r#type());
    let res = sqlx::query!(
        "UPDATE groups SET created_at = ?, updated_at = ?, name = ?, type = ? WHERE id = ?",
        created_at,
        updated_at,
        name,
        type_,
        id,
    )
    .execute(executor)
    .await?;
    Ok(res.rows_affected())
}

async fn exec_delete<'c, E>(executor: E, id: GroupId) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = id.to_string();
    let res = sqlx::query!("DELETE FROM groups WHERE id = ?", id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

#[async_trait]
impl GroupRepo<SqliteTransaction> for SqliteGroupRepo {
    async fn find_by_id(&self, id: GroupId) -> Result<Option<Group>, FindError> {
        let id = id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, created_at, updated_at, name, type FROM groups WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<Group> {
            Group::restore(
                GroupId::from_str(&r.id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                ms_to_dt(r.created_at)?,
                ms_to_dt(r.updated_at)?,
                r.name,
                group_type_from_tag(&r.r#type)?,
            )
            .map_err(|e| anyhow::anyhow!(e.to_string()))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_all(&self) -> Result<Vec<Group>, FindError> {
        let rows = sqlx::query!(r#"SELECT id, created_at, updated_at, name, type FROM groups"#,)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Group> {
                Group::restore(
                    GroupId::from_str(&r.id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                    ms_to_dt(r.created_at)?,
                    ms_to_dt(r.updated_at)?,
                    r.name,
                    group_type_from_tag(&r.r#type)?,
                )
                .map_err(|e| anyhow::anyhow!(e.to_string()))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, group: Group) -> Result<(), InsertError> {
        exec_insert(&self.pool, &group)
            .await
            .map_err(to_insert_error)
    }

    async fn insert_in(&self, tx: &mut SqliteTransaction, group: Group) -> Result<(), InsertError> {
        let conn = tx.conn().map_err(InsertError::InternalError)?;
        exec_insert(conn, &group).await.map_err(to_insert_error)
    }

    async fn update(&self, group: Group) -> Result<(), UpdateError> {
        let affected = exec_update(&self.pool, &group)
            .await
            .map_err(|e| UpdateError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn update_in(&self, tx: &mut SqliteTransaction, group: Group) -> Result<(), UpdateError> {
        let conn = tx.conn().map_err(UpdateError::InternalError)?;
        let affected = exec_update(conn, &group)
            .await
            .map_err(|e| UpdateError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn delete(&self, id: GroupId) -> Result<(), DeleteError> {
        let affected = exec_delete(&self.pool, id)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(&self, tx: &mut SqliteTransaction, id: GroupId) -> Result<(), DeleteError> {
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
