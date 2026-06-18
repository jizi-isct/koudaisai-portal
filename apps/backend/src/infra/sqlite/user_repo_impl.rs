use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::domain::email_address::EmailAddress;
use crate::domain::password_credentials::PasswordCredentials;
use crate::domain::user::{User, UserStatus};
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt, to_insert_error};
use async_trait::async_trait;
use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;

pub struct SqliteUserRepo {
    pool: SqlitePool,
}

impl SqliteUserRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// `UserStatus`(代数的データ型)をテーブルの各列へ分解した中間表現。
struct StatusCols {
    status: &'static str,
    password_phc: Option<String>,
    password_changed_at: Option<i64>,
    deactivated_at: Option<i64>,
    deactivation_reason: Option<String>,
}

fn status_cols(user: &User) -> StatusCols {
    match user.status() {
        UserStatus::Registered => StatusCols {
            status: "registered",
            password_phc: None,
            password_changed_at: None,
            deactivated_at: None,
            deactivation_reason: None,
        },
        UserStatus::Active {
            password_credentials,
        } => StatusCols {
            status: "active",
            password_phc: Some(password_credentials.phc().to_string()),
            password_changed_at: Some(dt_to_ms(*password_credentials.changed_at())),
            deactivated_at: None,
            deactivation_reason: None,
        },
        UserStatus::Deactivated {
            password_credentials,
            deactivated_at,
            reason,
        } => StatusCols {
            status: "deactivated",
            password_phc: Some(password_credentials.phc().to_string()),
            password_changed_at: Some(dt_to_ms(*password_credentials.changed_at())),
            deactivated_at: Some(dt_to_ms(*deactivated_at)),
            deactivation_reason: Some(reason.clone()),
        },
    }
}

/// status 列群から `UserStatus`(代数的データ型)を復元する。
fn user_status_from_cols(
    status: String,
    password_phc: Option<String>,
    password_changed_at: Option<i64>,
    deactivated_at: Option<i64>,
    deactivation_reason: Option<String>,
) -> anyhow::Result<UserStatus> {
    Ok(match status.as_str() {
        "registered" => UserStatus::Registered,
        "active" => {
            let phc =
                password_phc.ok_or_else(|| anyhow::anyhow!("active user missing password_phc"))?;
            let changed_at = ms_to_dt(
                password_changed_at
                    .ok_or_else(|| anyhow::anyhow!("active user missing password_changed_at"))?,
            )?;
            UserStatus::Active {
                password_credentials: PasswordCredentials::restore(&phc, changed_at)
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?,
            }
        }
        "deactivated" => {
            let phc = password_phc
                .ok_or_else(|| anyhow::anyhow!("deactivated user missing password_phc"))?;
            let changed_at =
                ms_to_dt(password_changed_at.ok_or_else(|| {
                    anyhow::anyhow!("deactivated user missing password_changed_at")
                })?)?;
            let deactivated_at = ms_to_dt(
                deactivated_at
                    .ok_or_else(|| anyhow::anyhow!("deactivated user missing deactivated_at"))?,
            )?;
            let reason = deactivation_reason
                .ok_or_else(|| anyhow::anyhow!("deactivated user missing deactivation_reason"))?;
            UserStatus::Deactivated {
                password_credentials: PasswordCredentials::restore(&phc, changed_at)
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?,
                deactivated_at,
                reason,
            }
        }
        other => return Err(anyhow::anyhow!("unknown user status: {}", other)),
    })
}

async fn exec_insert<'c, E>(executor: E, user: &User) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let c = status_cols(user);
    let id = Uuid::from(user.id()).to_string();
    let created_at = dt_to_ms(*user.created_at());
    let updated_at = dt_to_ms(*user.updated_at());
    let name = user.name().to_string();
    let m_address = user.m_address().address.clone();
    sqlx::query!(
        "INSERT INTO users \
         (id, created_at, updated_at, name, m_address, status, password_phc, password_changed_at, deactivated_at, deactivation_reason) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        id,
        created_at,
        updated_at,
        name,
        m_address,
        c.status,
        c.password_phc,
        c.password_changed_at,
        c.deactivated_at,
        c.deactivation_reason,
    )
    .execute(executor)
    .await?;
    Ok(())
}

async fn exec_update<'c, E>(executor: E, user: &User) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let c = status_cols(user);
    let id = Uuid::from(user.id()).to_string();
    let created_at = dt_to_ms(*user.created_at());
    let updated_at = dt_to_ms(*user.updated_at());
    let name = user.name().to_string();
    let m_address = user.m_address().address.clone();
    let res = sqlx::query!(
        "UPDATE users SET \
         created_at = ?, updated_at = ?, name = ?, m_address = ?, status = ?, \
         password_phc = ?, password_changed_at = ?, deactivated_at = ?, deactivation_reason = ? \
         WHERE id = ?",
        created_at,
        updated_at,
        name,
        m_address,
        c.status,
        c.password_phc,
        c.password_changed_at,
        c.deactivated_at,
        c.deactivation_reason,
        id,
    )
    .execute(executor)
    .await?;
    Ok(res.rows_affected())
}

async fn exec_delete<'c, E>(executor: E, id: UserId) -> Result<u64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = Uuid::from(id).to_string();
    let res = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

#[async_trait]
impl UserRepo<SqliteTransaction> for SqliteUserRepo {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, FindError> {
        let id = Uuid::from(id).to_string();
        let row = sqlx::query!(
            r#"SELECT id, created_at, updated_at, name, m_address, status,
                      password_phc, password_changed_at, deactivated_at, deactivation_reason
               FROM users WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<User> {
            Ok(User::restore(
                UserId::new(Uuid::parse_str(&r.id)?),
                ms_to_dt(r.created_at)?,
                ms_to_dt(r.updated_at)?,
                r.name,
                EmailAddress::new(r.m_address).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                user_status_from_cols(
                    r.status,
                    r.password_phc,
                    r.password_changed_at,
                    r.deactivated_at,
                    r.deactivation_reason,
                )?,
            ))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_by_m_address(&self, m_address: &EmailAddress) -> Result<Option<User>, FindError> {
        let addr = m_address.address.clone();
        let row = sqlx::query!(
            r#"SELECT id, created_at, updated_at, name, m_address, status,
                      password_phc, password_changed_at, deactivated_at, deactivation_reason
               FROM users WHERE m_address = ?"#,
            addr,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| -> anyhow::Result<User> {
            Ok(User::restore(
                UserId::new(Uuid::parse_str(&r.id)?),
                ms_to_dt(r.created_at)?,
                ms_to_dt(r.updated_at)?,
                r.name,
                EmailAddress::new(r.m_address).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                user_status_from_cols(
                    r.status,
                    r.password_phc,
                    r.password_changed_at,
                    r.deactivated_at,
                    r.deactivation_reason,
                )?,
            ))
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_all(&self) -> Result<Vec<User>, FindError> {
        let rows = sqlx::query!(
            r#"SELECT id, created_at, updated_at, name, m_address, status,
                      password_phc, password_changed_at, deactivated_at, deactivation_reason
               FROM users"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| -> anyhow::Result<User> {
                Ok(User::restore(
                    UserId::new(Uuid::parse_str(&r.id)?),
                    ms_to_dt(r.created_at)?,
                    ms_to_dt(r.updated_at)?,
                    r.name,
                    EmailAddress::new(r.m_address).map_err(|e| anyhow::anyhow!(e.to_string()))?,
                    user_status_from_cols(
                        r.status,
                        r.password_phc,
                        r.password_changed_at,
                        r.deactivated_at,
                        r.deactivation_reason,
                    )?,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert(&self, user: &User) -> Result<(), InsertError> {
        exec_insert(&self.pool, user).await.map_err(to_insert_error)
    }

    async fn insert_in(
        &self,
        tx: &mut SqliteTransaction,
        user: &User,
    ) -> Result<(), anyhow::Error> {
        exec_insert(tx.conn()?, user).await.map_err(Into::into)
    }

    async fn update(&self, user: &User) -> Result<(), UpdateError> {
        let affected = exec_update(&self.pool, user)
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
        user: &User,
    ) -> Result<(), anyhow::Error> {
        let affected = exec_update(tx.conn()?, user).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!("user not found: {}", user.id()));
        }
        Ok(())
    }

    async fn rehash_password_in(
        &self,
        tx: &mut SqliteTransaction,
        id: UserId,
        expected_old_phc: &str,
        new_phc: &str,
    ) -> Result<u64, anyhow::Error> {
        let id = Uuid::from(id).to_string();
        let res = sqlx::query!(
            "UPDATE users SET password_phc = ? \
             WHERE id = ? AND status = 'active' AND password_phc = ?",
            new_phc,
            id,
            expected_old_phc,
        )
        .execute(tx.conn()?)
        .await?;
        Ok(res.rows_affected())
    }

    async fn delete(&self, id: UserId) -> Result<(), DeleteError> {
        let affected = exec_delete(&self.pool, id)
            .await
            .map_err(|e| DeleteError::InternalError(e.into()))?;
        if affected == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(&self, tx: &mut SqliteTransaction, id: UserId) -> Result<(), anyhow::Error> {
        let affected = exec_delete(tx.conn()?, id).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!("user not found: {}", id));
        }
        Ok(())
    }
}
