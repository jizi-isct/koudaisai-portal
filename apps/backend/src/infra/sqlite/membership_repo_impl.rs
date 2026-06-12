use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::domain::group_id::GroupId;
use crate::domain::membership::Membership;
use crate::domain::user_id::UserId;
use crate::infra::clock_impl::ClockImpl;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::to_insert_error;
use async_trait::async_trait;
use sqlx::{Sqlite, SqlitePool};
use std::str::FromStr;
use uuid::Uuid;

pub struct SqliteMembershipRepo {
    pool: SqlitePool,
}

impl SqliteMembershipRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

async fn exec_insert<'c, E>(executor: E, membership: &Membership) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let user_id = Uuid::from(membership.user_id()).to_string();
    let group_id = membership.group_id().to_string();
    sqlx::query!(
        "INSERT INTO memberships (user_id, group_id) VALUES (?, ?)",
        user_id,
        group_id,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_exists<'c, E>(executor: E, membership: &Membership) -> Result<bool, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let user_id = Uuid::from(membership.user_id()).to_string();
    let group_id = membership.group_id().to_string();
    let row = sqlx::query!(
        "SELECT user_id FROM memberships WHERE user_id = ? AND group_id = ?",
        user_id,
        group_id,
    )
    .fetch_optional(executor)
    .await?;
    Ok(row.is_some())
}

async fn exec_delete<'c, E>(executor: E, membership: &Membership) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let user_id = Uuid::from(membership.user_id()).to_string();
    let group_id = membership.group_id().to_string();
    let res = sqlx::query!(
        "DELETE FROM memberships WHERE user_id = ? AND group_id = ?",
        user_id,
        group_id,
    )
    .execute(executor)
    .await?;
    Ok(res.rows_affected())
}

#[async_trait]
impl MembershipRepo<SqliteTransaction> for SqliteMembershipRepo {
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Membership>, FindError> {
        let user_id = Uuid::from(user_id).to_string();
        let rows = sqlx::query!(
            r#"SELECT user_id, group_id FROM memberships WHERE user_id = ?"#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Membership> {
                // Membership::new は clock を保持しないため，任意の Clock 実装で復元できる。
                Ok(Membership::new(
                    GroupId::from_str(&r.group_id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                    UserId::new(Uuid::parse_str(&r.user_id)?),
                    &ClockImpl,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn find_by_group_id(&self, group_id: GroupId) -> Result<Vec<Membership>, FindError> {
        let group_id = group_id.to_string();
        let rows = sqlx::query!(
            r#"SELECT user_id, group_id FROM memberships WHERE group_id = ?"#,
            group_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Membership> {
                // Membership::new は clock を保持しないため，任意の Clock 実装で復元できる。
                Ok(Membership::new(
                    GroupId::from_str(&r.group_id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                    UserId::new(Uuid::parse_str(&r.user_id)?),
                    &ClockImpl,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, membership: Membership) -> Result<(), InsertError> {
        exec_insert(&self.pool, &membership).await.map_err(to_insert_error)
    }

    async fn insert_in(
        &self,
        tx: &mut SqliteTransaction,
        membership: Membership,
    ) -> Result<(), InsertError> {
        let conn = tx.conn().map_err(InsertError::InternalError)?;
        exec_insert(conn, &membership).await.map_err(to_insert_error)
    }

    async fn update(&self, membership: Membership) -> Result<(), UpdateError> {
        // memberships は (user_id, group_id) のみで非キー列を持たないため，
        // 更新は存在確認のみを行う。
        let exists = exec_exists(&self.pool, &membership)
            .await
            .map_err(|e| UpdateError::InternalError(e.into()))?;
        if !exists {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn update_in(
        &self,
        tx: &mut SqliteTransaction,
        membership: Membership,
    ) -> Result<(), UpdateError> {
        let conn = tx.conn().map_err(UpdateError::InternalError)?;
        let exists = exec_exists(conn, &membership)
            .await
            .map_err(|e| UpdateError::InternalError(e.into()))?;
        if !exists {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }

    async fn delete(&self, membership: Membership) -> Result<(), DeleteError> {
        let affected = exec_delete(&self.pool, &membership)
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
        membership: Membership,
    ) -> Result<(), DeleteError> {
        let conn = tx.conn().map_err(DeleteError::InternalError)?;
        let affected = exec_delete(conn, &membership)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }
}
