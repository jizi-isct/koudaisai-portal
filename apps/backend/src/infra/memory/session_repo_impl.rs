//! インメモリの [`SessionRepo`]。sqlite 版とテスト等価になるよう挙動を忠実に再現する。
//! ドメインのフィールドは非公開なので，状態遷移は `restore` で組み直して反映する。
//!
//! ロックは常に「単一ロックを取得→使用→解放」の逐次取得とし，デッドロックを避ける。

use crate::application::error::FindError;
use crate::application::ports::repositories::session_repo::SessionRepo;
use crate::domain::session::{RevocationReason, Session, SessionState, SessionToken, TokenStatus};
use crate::domain::session_id::SessionId;
use crate::domain::token_id::TokenId;
use crate::domain::user_id::UserId;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct MemorySessionRepo {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
    tokens: Arc<RwLock<HashMap<TokenId, SessionToken>>>,
}

impl MemorySessionRepo {
    pub fn new() -> Self {
        Self::default()
    }
}

/// セッションを失効状態へ組み直す。
fn revoked_session(s: &Session, reason: RevocationReason, at: DateTime<Utc>) -> Session {
    Session::restore(
        s.id(),
        s.user_id(),
        *s.created_at(),
        *s.absolute_expires_at(),
        SessionState::Revoked { at, reason },
    )
}

/// トークンのステータスだけを差し替えて組み直す。
fn with_status(t: &SessionToken, status: TokenStatus) -> SessionToken {
    SessionToken::restore(
        t.id(),
        t.session_id(),
        t.generation(),
        t.secret_hash().to_string(),
        *t.issued_at(),
        *t.idle_expires_at(),
        status,
    )
}

#[async_trait]
impl SessionRepo<MemoryTransaction> for MemorySessionRepo {
    async fn find_session_by_id(&self, id: SessionId) -> Result<Option<Session>, FindError> {
        let sessions = self
            .sessions
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(sessions.get(&id).cloned())
    }

    async fn find_token_by_id(&self, id: TokenId) -> Result<Option<SessionToken>, FindError> {
        let tokens = self
            .tokens
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(tokens.get(&id).cloned())
    }

    async fn find_active_sessions_by_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<Session>, FindError> {
        let sessions = self
            .sessions
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(sessions
            .values()
            .filter(|s| s.user_id() == user_id && s.is_active())
            .cloned()
            .collect())
    }

    async fn insert_session_in(
        &self,
        _tx: &mut MemoryTransaction,
        session: &Session,
        first: &SessionToken,
    ) -> Result<(), anyhow::Error> {
        {
            let mut sessions = self.sessions.write().map_err(|e| anyhow!(e.to_string()))?;
            if sessions.contains_key(&session.id()) {
                return Err(anyhow!("session id conflict: {}", session.id()));
            }
            sessions.insert(session.id(), session.clone());
        }
        {
            let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
            if tokens.contains_key(&first.id()) {
                return Err(anyhow!("session token id conflict: {}", first.id()));
            }
            tokens.insert(first.id(), first.clone());
        }
        Ok(())
    }

    async fn rotate_in(
        &self,
        _tx: &mut MemoryTransaction,
        old_token_id: TokenId,
        new: &SessionToken,
    ) -> Result<u64, anyhow::Error> {
        let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
        // CAS: 旧世代が現存し，かつ Issued のときだけ回転できる。
        match tokens.get(&old_token_id) {
            Some(old) if old.status() == TokenStatus::Issued => {
                let rotated = with_status(old, TokenStatus::Rotated);
                tokens.insert(old_token_id, rotated);
                tokens.insert(new.id(), new.clone());
                Ok(1)
            }
            _ => Ok(0),
        }
    }

    async fn revoke_family_in(
        &self,
        _tx: &mut MemoryTransaction,
        session_id: SessionId,
        reason: RevocationReason,
        at: DateTime<Utc>,
    ) -> Result<(), anyhow::Error> {
        {
            let mut sessions = self.sessions.write().map_err(|e| anyhow!(e.to_string()))?;
            if let Some(s) = sessions.get(&session_id)
                && s.is_active()
            {
                let revoked = revoked_session(s, reason, at);
                sessions.insert(session_id, revoked);
            }
        }
        {
            let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
            let ids: Vec<TokenId> = tokens
                .values()
                .filter(|t| t.session_id() == session_id && t.status() != TokenStatus::Revoked)
                .map(|t| t.id())
                .collect();
            for id in ids {
                if let Some(t) = tokens.get(&id) {
                    let revoked = with_status(t, TokenStatus::Revoked);
                    tokens.insert(id, revoked);
                }
            }
        }
        Ok(())
    }

    async fn revoke_all_for_user_in(
        &self,
        _tx: &mut MemoryTransaction,
        user_id: UserId,
        reason: RevocationReason,
        at: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error> {
        // トークン失効のスコープは「そのユーザの全セッション」(sqlite の副問い合わせと等価)。
        // 返り値は active→revoked にしたセッション数(sqlite の rows_affected と等価)。
        let all_user_sessions: HashSet<SessionId>;
        let flipped: usize;
        {
            let mut sessions = self.sessions.write().map_err(|e| anyhow!(e.to_string()))?;
            all_user_sessions = sessions
                .values()
                .filter(|s| s.user_id() == user_id)
                .map(|s| s.id())
                .collect();
            let active_ids: Vec<SessionId> = sessions
                .values()
                .filter(|s| s.user_id() == user_id && s.is_active())
                .map(|s| s.id())
                .collect();
            for id in &active_ids {
                if let Some(s) = sessions.get(id) {
                    let revoked = revoked_session(s, reason, at);
                    sessions.insert(*id, revoked);
                }
            }
            flipped = active_ids.len();
        }
        {
            let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
            let ids: Vec<TokenId> = tokens
                .values()
                .filter(|t| {
                    all_user_sessions.contains(&t.session_id())
                        && t.status() != TokenStatus::Revoked
                })
                .map(|t| t.id())
                .collect();
            for id in ids {
                if let Some(t) = tokens.get(&id) {
                    let revoked = with_status(t, TokenStatus::Revoked);
                    tokens.insert(id, revoked);
                }
            }
        }
        Ok(flipped as u64)
    }

    async fn delete_expired(&self, now: DateTime<Utc>) -> Result<u64, anyhow::Error> {
        // 失効済み or 絶対期限切れのファミリだけを削除する(Active ファミリの Rotated 行は残す)。
        let removed: HashSet<SessionId>;
        {
            let mut sessions = self.sessions.write().map_err(|e| anyhow!(e.to_string()))?;
            let ids: Vec<SessionId> = sessions
                .values()
                .filter(|s| !s.is_active() || now >= *s.absolute_expires_at())
                .map(|s| s.id())
                .collect();
            for id in &ids {
                sessions.remove(id);
            }
            removed = ids.into_iter().collect();
        }
        {
            let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
            tokens.retain(|_, t| !removed.contains(&t.session_id()));
        }
        Ok(removed.len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::clock::Clock;
    use chrono::{Duration, TimeZone};
    use uuid::Uuid;

    struct MockClock {
        now: DateTime<Utc>,
    }
    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    fn start(now: DateTime<Utc>) -> (Session, SessionToken) {
        Session::start(
            SessionId::new(Uuid::new_v4()),
            TokenId::new(Uuid::new_v4()),
            UserId::new(Uuid::new_v4()),
            "hash0".to_string(),
            Duration::days(180),
            Duration::days(14),
            &MockClock { now },
        )
    }

    #[tokio::test]
    async fn rotate_in_is_cas_single_winner() {
        let repo = MemorySessionRepo::new();
        let mut tx = MemoryTransaction::new();
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let (session, token0) = start(t0);
        repo.insert_session_in(&mut tx, &session, &token0)
            .await
            .unwrap();

        let next = token0.rotate(
            TokenId::new(Uuid::new_v4()),
            "hash1".to_string(),
            Duration::days(14),
            *session.absolute_expires_at(),
            &MockClock {
                now: t0 + Duration::minutes(1),
            },
        );
        // 1 回目の回転は成功(1)。
        assert_eq!(
            repo.rotate_in(&mut tx, token0.id(), &next).await.unwrap(),
            1
        );
        // 同じ旧世代での再回転は CAS により 0(並行回転/reuse)。
        let next2 = token0.rotate(
            TokenId::new(Uuid::new_v4()),
            "hash2".to_string(),
            Duration::days(14),
            *session.absolute_expires_at(),
            &MockClock {
                now: t0 + Duration::minutes(2),
            },
        );
        assert_eq!(
            repo.rotate_in(&mut tx, token0.id(), &next2).await.unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn revoke_family_revokes_session_and_tokens() {
        let repo = MemorySessionRepo::new();
        let mut tx = MemoryTransaction::new();
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let (session, token0) = start(t0);
        repo.insert_session_in(&mut tx, &session, &token0)
            .await
            .unwrap();

        repo.revoke_family_in(&mut tx, session.id(), RevocationReason::ReuseDetected, t0)
            .await
            .unwrap();

        let s = repo
            .find_session_by_id(session.id())
            .await
            .unwrap()
            .unwrap();
        assert!(!s.is_active());
        let t = repo.find_token_by_id(token0.id()).await.unwrap().unwrap();
        assert_eq!(t.status(), TokenStatus::Revoked);
    }

    #[tokio::test]
    async fn delete_expired_keeps_active_rotated_tokens() {
        let repo = MemorySessionRepo::new();
        let mut tx = MemoryTransaction::new();
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let (session, token0) = start(t0);
        repo.insert_session_in(&mut tx, &session, &token0)
            .await
            .unwrap();
        // 回転して旧世代を Rotated にする(ファミリは Active のまま)。
        let next = token0.rotate(
            TokenId::new(Uuid::new_v4()),
            "hash1".to_string(),
            Duration::days(14),
            *session.absolute_expires_at(),
            &MockClock {
                now: t0 + Duration::minutes(1),
            },
        );
        repo.rotate_in(&mut tx, token0.id(), &next).await.unwrap();

        // 絶対期限内の掃除では Active ファミリの Rotated 行は消えない。
        assert_eq!(
            repo.delete_expired(t0 + Duration::days(1)).await.unwrap(),
            0
        );
        assert!(repo.find_token_by_id(token0.id()).await.unwrap().is_some());

        // 絶対期限を過ぎたら丸ごと掃除される。
        assert_eq!(
            repo.delete_expired(t0 + Duration::days(181)).await.unwrap(),
            1
        );
        assert!(repo.find_token_by_id(token0.id()).await.unwrap().is_none());
        assert!(
            repo.find_session_by_id(session.id())
                .await
                .unwrap()
                .is_none()
        );
    }
}
