use crate::application::error::FindError;
use crate::application::ports::repositories::one_time_token_repo::OneTimeTokenRepo;
use crate::domain::email_address::EmailAddress;
use crate::domain::one_time_token::{OneTimeToken, OneTimeTokenPurpose};
use crate::domain::one_time_token_id::OneTimeTokenId;
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteOneTimeTokenRepo {
    pool: SqlitePool,
}

impl SqliteOneTimeTokenRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[allow(clippy::too_many_arguments)]
fn to_domain(
    id: String,
    user_id: String,
    purpose: String,
    token_hash: String,
    created_at: i64,
    expires_at: i64,
    consumed_at: Option<i64>,
    m_address: Option<String>,
) -> anyhow::Result<OneTimeToken> {
    Ok(OneTimeToken::restore(
        OneTimeTokenId::new(Uuid::parse_str(&id)?),
        UserId::new(Uuid::parse_str(&user_id)?),
        OneTimeTokenPurpose::from_str(&purpose)
            .ok_or_else(|| anyhow::anyhow!("unknown one-time token purpose: {purpose}"))?,
        token_hash,
        ms_to_dt(created_at)?,
        ms_to_dt(expires_at)?,
        consumed_at.map(ms_to_dt).transpose()?,
        m_address
            .map(EmailAddress::new)
            .transpose()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?,
    ))
}

#[async_trait]
impl OneTimeTokenRepo<SqliteTransaction> for SqliteOneTimeTokenRepo {
    async fn find_by_id(&self, id: OneTimeTokenId) -> Result<Option<OneTimeToken>, FindError> {
        let id = Uuid::from(id).to_string();
        let row = sqlx::query!(
            r#"SELECT id, user_id, purpose, token_hash, created_at, expires_at, consumed_at, m_address
               FROM one_time_tokens WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| {
            to_domain(
                r.id,
                r.user_id,
                r.purpose,
                r.token_hash,
                r.created_at,
                r.expires_at,
                r.consumed_at,
                r.m_address,
            )
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn insert_in(
        &self,
        tx: &mut SqliteTransaction,
        token: &OneTimeToken,
    ) -> Result<(), anyhow::Error> {
        let id = Uuid::from(token.id()).to_string();
        let user_id = Uuid::from(token.user_id()).to_string();
        let purpose = token.purpose().as_str();
        let token_hash = token.token_hash().to_string();
        let created_at = dt_to_ms(*token.created_at());
        let expires_at = dt_to_ms(*token.expires_at());
        let consumed_at = token.consumed_at().map(|d| dt_to_ms(*d));
        let m_address = token.m_address().map(|e| e.address.clone());
        sqlx::query!(
            "INSERT INTO one_time_tokens \
             (id, user_id, purpose, token_hash, created_at, expires_at, consumed_at, m_address) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            id,
            user_id,
            purpose,
            token_hash,
            created_at,
            expires_at,
            consumed_at,
            m_address,
        )
        .execute(tx.conn()?)
        .await?;
        Ok(())
    }

    async fn consume_in(
        &self,
        tx: &mut SqliteTransaction,
        id: OneTimeTokenId,
        now: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error> {
        let id = Uuid::from(id).to_string();
        let now = dt_to_ms(now);
        let res = sqlx::query!(
            "UPDATE one_time_tokens SET consumed_at = ? \
             WHERE id = ? AND consumed_at IS NULL AND expires_at > ?",
            now,
            id,
            now,
        )
        .execute(tx.conn()?)
        .await?;
        Ok(res.rows_affected())
    }

    async fn invalidate_existing_for_in(
        &self,
        tx: &mut SqliteTransaction,
        user_id: UserId,
        purpose: OneTimeTokenPurpose,
        now: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error> {
        let user_id = Uuid::from(user_id).to_string();
        let purpose = purpose.as_str();
        let now = dt_to_ms(now);
        let res = sqlx::query!(
            "UPDATE one_time_tokens SET consumed_at = ? \
             WHERE user_id = ? AND purpose = ? AND consumed_at IS NULL",
            now,
            user_id,
            purpose,
        )
        .execute(tx.conn()?)
        .await?;
        Ok(res.rows_affected())
    }

    async fn delete_expired(&self, now: DateTime<Utc>) -> Result<u64, anyhow::Error> {
        let now = dt_to_ms(now);
        let res = sqlx::query!("DELETE FROM one_time_tokens WHERE expires_at <= ?", now)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }
}
