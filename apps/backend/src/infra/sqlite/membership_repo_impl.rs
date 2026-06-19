use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::domain::group_id::GroupId;
use crate::domain::membership::{Membership, Role};
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt, to_insert_error};
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

/// `Role` を DB 上の snake_case 文字列へ変換する。
fn role_to_db(role: Role) -> &'static str {
    match role {
        Role::Representative => "representative",
        Role::Operator => "operator",
        Role::FirstResponsible => "first_responsible",
        Role::SecondResponsible => "second_responsible",
        Role::ThirdResponsible => "third_responsible",
    }
}

/// DB 上の snake_case 文字列を `Role` へ変換する。
fn role_from_db(s: &str) -> anyhow::Result<Role> {
    match s {
        "representative" => Ok(Role::Representative),
        "operator" => Ok(Role::Operator),
        "first_responsible" => Ok(Role::FirstResponsible),
        "second_responsible" => Ok(Role::SecondResponsible),
        "third_responsible" => Ok(Role::ThirdResponsible),
        other => Err(anyhow::anyhow!("unknown role: {}", other)),
    }
}

async fn exec_insert<'c, E>(executor: E, membership: &Membership) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let group_id = membership.group_id().to_string();
    let user_id = Uuid::from(membership.user_id()).to_string();
    let role = role_to_db(membership.role());
    let created_at = dt_to_ms(membership.created_at());
    sqlx::query!(
        "INSERT INTO memberships (group_id, user_id, role, created_at) VALUES (?, ?, ?, ?)",
        group_id,
        user_id,
        role,
        created_at,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_exists<'c, E>(executor: E, membership: &Membership) -> Result<bool, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let group_id = membership.group_id().to_string();
    let role = role_to_db(membership.role());
    let row = sqlx::query!(
        "SELECT group_id FROM memberships WHERE group_id = ? AND role = ?",
        group_id,
        role,
    )
    .fetch_optional(executor)
    .await?;
    Ok(row.is_some())
}

async fn exec_delete<'c, E>(executor: E, membership: &Membership) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let group_id = membership.group_id().to_string();
    let role = role_to_db(membership.role());
    let res = sqlx::query!(
        "DELETE FROM memberships WHERE group_id = ? AND role = ?",
        group_id,
        role,
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
            r#"SELECT group_id, user_id, role, created_at FROM memberships WHERE user_id = ?"#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Membership> {
                Ok(Membership::restore(
                    GroupId::from_str(&r.group_id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                    UserId::new(Uuid::parse_str(&r.user_id)?),
                    role_from_db(&r.role)?,
                    ms_to_dt(r.created_at)?,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn find_by_group_id(&self, group_id: GroupId) -> Result<Vec<Membership>, FindError> {
        let group_id = group_id.to_string();
        let rows = sqlx::query!(
            r#"SELECT group_id, user_id, role, created_at FROM memberships WHERE group_id = ?"#,
            group_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<Membership> {
                Ok(Membership::restore(
                    GroupId::from_str(&r.group_id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                    UserId::new(Uuid::parse_str(&r.user_id)?),
                    role_from_db(&r.role)?,
                    ms_to_dt(r.created_at)?,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, membership: Membership) -> Result<(), InsertError> {
        exec_insert(&self.pool, &membership)
            .await
            .map_err(to_insert_error)
    }

    async fn insert_in(
        &self,
        tx: &mut SqliteTransaction,
        membership: Membership,
    ) -> Result<(), InsertError> {
        let conn = tx.conn().map_err(InsertError::InternalError)?;
        exec_insert(conn, &membership)
            .await
            .map_err(to_insert_error)
    }

    async fn update(&self, membership: Membership) -> Result<(), UpdateError> {
        // memberships は (group_id, role) を主キーとし，role 変更は remove+add で
        // 表現するため実質的に不変である。更新は (group_id, role) の存在確認のみを行う。
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
