use crate::application::error::FindError;
use crate::application::ports::repositories::session_repo::SessionRepo;
use crate::domain::session::{RevocationReason, Session, SessionState, SessionToken, TokenStatus};
use crate::domain::session_id::SessionId;
use crate::domain::token_id::TokenId;
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::util::{dt_to_ms, ms_to_dt};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;

pub struct SqliteSessionRepo {
    pool: SqlitePool,
}

impl SqliteSessionRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

struct SessionStateCols {
    state: &'static str,
    revoked_at: Option<i64>,
    revocation_reason: Option<&'static str>,
}

fn session_state_cols(s: &Session) -> SessionStateCols {
    match s.state() {
        SessionState::Active => SessionStateCols {
            state: "active",
            revoked_at: None,
            revocation_reason: None,
        },
        SessionState::Revoked { at, reason } => SessionStateCols {
            state: "revoked",
            revoked_at: Some(dt_to_ms(*at)),
            revocation_reason: Some(reason.as_str()),
        },
    }
}

fn session_from_row(
    id: String,
    user_id: String,
    created_at: i64,
    absolute_expires_at: i64,
    state: String,
    revoked_at: Option<i64>,
    revocation_reason: Option<String>,
) -> anyhow::Result<Session> {
    let state = match state.as_str() {
        "active" => SessionState::Active,
        "revoked" => SessionState::Revoked {
            at: ms_to_dt(
                revoked_at.ok_or_else(|| anyhow::anyhow!("revoked session missing revoked_at"))?,
            )?,
            reason: RevocationReason::from_str(
                &revocation_reason
                    .ok_or_else(|| anyhow::anyhow!("revoked session missing revocation_reason"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("unknown revocation reason"))?,
        },
        other => return Err(anyhow::anyhow!("unknown session state: {other}")),
    };
    Ok(Session::restore(
        SessionId::new(Uuid::parse_str(&id)?),
        UserId::new(Uuid::parse_str(&user_id)?),
        ms_to_dt(created_at)?,
        ms_to_dt(absolute_expires_at)?,
        state,
    ))
}

fn token_from_row(
    id: String,
    session_id: String,
    generation: i64,
    secret_hash: String,
    issued_at: i64,
    idle_expires_at: i64,
    status: String,
) -> anyhow::Result<SessionToken> {
    Ok(SessionToken::restore(
        TokenId::new(Uuid::parse_str(&id)?),
        SessionId::new(Uuid::parse_str(&session_id)?),
        u32::try_from(generation).map_err(|_| anyhow::anyhow!("generation out of range"))?,
        secret_hash,
        ms_to_dt(issued_at)?,
        ms_to_dt(idle_expires_at)?,
        TokenStatus::from_str(&status)
            .ok_or_else(|| anyhow::anyhow!("unknown token status: {status}"))?,
    ))
}

async fn insert_token<'c, E>(executor: E, t: &SessionToken) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
{
    let id = Uuid::from(t.id()).to_string();
    let session_id = Uuid::from(t.session_id()).to_string();
    let generation = t.generation() as i64;
    let secret_hash = t.secret_hash().to_string();
    let issued_at = dt_to_ms(*t.issued_at());
    let idle_expires_at = dt_to_ms(*t.idle_expires_at());
    let status = t.status().as_str();
    sqlx::query!(
        "INSERT INTO session_tokens \
         (id, session_id, generation, secret_hash, issued_at, idle_expires_at, status) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        id,
        session_id,
        generation,
        secret_hash,
        issued_at,
        idle_expires_at,
        status,
    )
    .execute(executor)
    .await?;
    Ok(())
}

#[async_trait]
impl SessionRepo<SqliteTransaction> for SqliteSessionRepo {
    async fn find_session_by_id(&self, id: SessionId) -> Result<Option<Session>, FindError> {
        let id = Uuid::from(id).to_string();
        let row = sqlx::query!(
            r#"SELECT id, user_id, created_at, absolute_expires_at, state, revoked_at, revocation_reason
               FROM sessions WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| {
            session_from_row(
                r.id,
                r.user_id,
                r.created_at,
                r.absolute_expires_at,
                r.state,
                r.revoked_at,
                r.revocation_reason,
            )
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_token_by_id(&self, id: TokenId) -> Result<Option<SessionToken>, FindError> {
        let id = Uuid::from(id).to_string();
        let row = sqlx::query!(
            r#"SELECT id, session_id, generation, secret_hash, issued_at, idle_expires_at, status
               FROM session_tokens WHERE id = ?"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        row.map(|r| {
            token_from_row(
                r.id,
                r.session_id,
                r.generation,
                r.secret_hash,
                r.issued_at,
                r.idle_expires_at,
                r.status,
            )
        })
        .transpose()
        .map_err(FindError::InternalError)
    }

    async fn find_active_sessions_by_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<Session>, FindError> {
        let user_id = Uuid::from(user_id).to_string();
        let rows = sqlx::query!(
            r#"SELECT id, user_id, created_at, absolute_expires_at, state, revoked_at, revocation_reason
               FROM sessions WHERE user_id = ? AND state = 'active'"#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FindError::InternalError(e.into()))?;
        rows.into_iter()
            .map(|r| {
                session_from_row(
                    r.id,
                    r.user_id,
                    r.created_at,
                    r.absolute_expires_at,
                    r.state,
                    r.revoked_at,
                    r.revocation_reason,
                )
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(FindError::InternalError)
    }

    async fn insert_session_in(
        &self,
        tx: &mut SqliteTransaction,
        session: &Session,
        first: &SessionToken,
    ) -> Result<(), anyhow::Error> {
        let c = session_state_cols(session);
        let id = Uuid::from(session.id()).to_string();
        let user_id = Uuid::from(session.user_id()).to_string();
        let created_at = dt_to_ms(*session.created_at());
        let absolute_expires_at = dt_to_ms(*session.absolute_expires_at());
        sqlx::query!(
            "INSERT INTO sessions \
             (id, user_id, created_at, absolute_expires_at, state, revoked_at, revocation_reason) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            id,
            user_id,
            created_at,
            absolute_expires_at,
            c.state,
            c.revoked_at,
            c.revocation_reason,
        )
        .execute(tx.conn()?)
        .await?;
        insert_token(tx.conn()?, first).await?;
        Ok(())
    }

    async fn rotate_in(
        &self,
        tx: &mut SqliteTransaction,
        old_token_id: TokenId,
        new: &SessionToken,
    ) -> Result<u64, anyhow::Error> {
        let old_id = Uuid::from(old_token_id).to_string();
        // CAS: status='issued' のときだけ rotated にできる。
        let res = sqlx::query!(
            "UPDATE session_tokens SET status = 'rotated' WHERE id = ? AND status = 'issued'",
            old_id,
        )
        .execute(tx.conn()?)
        .await?;
        if res.rows_affected() == 0 {
            return Ok(0);
        }
        insert_token(tx.conn()?, new).await?;
        Ok(1)
    }

    async fn revoke_family_in(
        &self,
        tx: &mut SqliteTransaction,
        session_id: SessionId,
        reason: RevocationReason,
        at: DateTime<Utc>,
    ) -> Result<(), anyhow::Error> {
        let sid = Uuid::from(session_id).to_string();
        let at_ms = dt_to_ms(at);
        let reason = reason.as_str();
        sqlx::query!(
            "UPDATE sessions SET state = 'revoked', revoked_at = ?, revocation_reason = ? \
             WHERE id = ? AND state = 'active'",
            at_ms,
            reason,
            sid,
        )
        .execute(tx.conn()?)
        .await?;
        sqlx::query!(
            "UPDATE session_tokens SET status = 'revoked' \
             WHERE session_id = ? AND status <> 'revoked'",
            sid,
        )
        .execute(tx.conn()?)
        .await?;
        Ok(())
    }

    async fn revoke_all_for_user_in(
        &self,
        tx: &mut SqliteTransaction,
        user_id: UserId,
        reason: RevocationReason,
        at: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error> {
        let uid = Uuid::from(user_id).to_string();
        let at_ms = dt_to_ms(at);
        let reason = reason.as_str();
        let res = sqlx::query!(
            "UPDATE sessions SET state = 'revoked', revoked_at = ?, revocation_reason = ? \
             WHERE user_id = ? AND state = 'active'",
            at_ms,
            reason,
            uid,
        )
        .execute(tx.conn()?)
        .await?;
        sqlx::query!(
            "UPDATE session_tokens SET status = 'revoked' \
             WHERE session_id IN (SELECT id FROM sessions WHERE user_id = ?) AND status <> 'revoked'",
            uid,
        )
        .execute(tx.conn()?)
        .await?;
        Ok(res.rows_affected())
    }

    async fn delete_expired(&self, now: DateTime<Utc>) -> Result<u64, anyhow::Error> {
        let now = dt_to_ms(now);
        // 失効済み or 絶対期限切れのファミリのみ削除(session_tokens は ON DELETE CASCADE)。
        // Active ファミリの rotated 行は残す(reuse 検知の盲点を作らない)。
        let res = sqlx::query!(
            "DELETE FROM sessions WHERE state = 'revoked' OR absolute_expires_at <= ?",
            now,
        )
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }
}
