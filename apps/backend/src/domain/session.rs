//! サーバーサイドの回転セッション。
//!
//! 「ファミリ([`Session`]) + 世代([`SessionToken`])」の2集約で表現する。
//! リフレッシュのたびに新しい世代を発行し，旧世代を `Rotated` にする。
//! **各世代が独自の `secret_hash` を持つ**ため，提示された旧 secret を検証でき，
//! 「secret が一致 かつ その世代が既に `Rotated`/`Revoked`」のときだけ盗難(reuse)と
//! 判定できる(token_id を知るだけでは他人のファミリを失効させられない)。

use crate::application::ports::clock::Clock;
use crate::domain::error::InvalidTransitionError;
use crate::domain::session_id::SessionId;
use crate::domain::token_id::TokenId;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Duration, Utc};

/// 失効理由。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevocationReason {
    Logout,
    LogoutAll,
    ReuseDetected,
    PasswordChanged,
    Deactivated,
    AdminRevoke,
    EvictedLru,
    Expired,
}

impl RevocationReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            RevocationReason::Logout => "logout",
            RevocationReason::LogoutAll => "logout_all",
            RevocationReason::ReuseDetected => "reuse_detected",
            RevocationReason::PasswordChanged => "password_changed",
            RevocationReason::Deactivated => "deactivated",
            RevocationReason::AdminRevoke => "admin_revoke",
            RevocationReason::EvictedLru => "evicted_lru",
            RevocationReason::Expired => "expired",
        }
    }

    // as_str と対になる `&str -> Option<Self>` パーサ。FromStr(Result 返し)とは別物。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "logout" => RevocationReason::Logout,
            "logout_all" => RevocationReason::LogoutAll,
            "reuse_detected" => RevocationReason::ReuseDetected,
            "password_changed" => RevocationReason::PasswordChanged,
            "deactivated" => RevocationReason::Deactivated,
            "admin_revoke" => RevocationReason::AdminRevoke,
            "evicted_lru" => RevocationReason::EvictedLru,
            "expired" => RevocationReason::Expired,
            _ => return None,
        })
    }
}

/// ファミリの状態。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Revoked {
        at: DateTime<Utc>,
        reason: RevocationReason,
    },
}

/// セッション(ファミリ)。絶対期限は発行時に固定し回転で延びない硬上限。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    id: SessionId,
    user_id: UserId,
    created_at: DateTime<Utc>,
    absolute_expires_at: DateTime<Utc>,
    state: SessionState,
}

/// 世代トークンのステータス。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenStatus {
    Issued,
    Rotated,
    Revoked,
}

impl TokenStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenStatus::Issued => "issued",
            TokenStatus::Rotated => "rotated",
            TokenStatus::Revoked => "revoked",
        }
    }

    // as_str と対になる `&str -> Option<Self>` パーサ。FromStr(Result 返し)とは別物。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "issued" => TokenStatus::Issued,
            "rotated" => TokenStatus::Rotated,
            "revoked" => TokenStatus::Revoked,
            _ => return None,
        })
    }
}

/// セッショントークン(世代)。各行が独自の `secret_hash` とアイドル期限を持つ。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionToken {
    id: TokenId,
    session_id: SessionId,
    generation: u32,
    secret_hash: String,
    issued_at: DateTime<Utc>,
    idle_expires_at: DateTime<Utc>,
    status: TokenStatus,
}

fn clamp_idle(now: DateTime<Utc>, idle_ttl: Duration, absolute: DateTime<Utc>) -> DateTime<Utc> {
    (now + idle_ttl).min(absolute)
}

impl Session {
    /// 新しいセッションと第0世代トークンを生成する。
    /// `secret_hash` と各 id は infra(SecretGenerator)で生成して渡す。
    #[allow(clippy::too_many_arguments)]
    pub fn start<C: Clock>(
        id: SessionId,
        first_token_id: TokenId,
        user_id: UserId,
        first_secret_hash: String,
        absolute_ttl: Duration,
        idle_ttl: Duration,
        clock: &C,
    ) -> (Session, SessionToken) {
        let now = clock.now();
        let absolute_expires_at = now + absolute_ttl;
        let session = Session {
            id,
            user_id,
            created_at: now,
            absolute_expires_at,
            state: SessionState::Active,
        };
        let token = SessionToken {
            id: first_token_id,
            session_id: id,
            generation: 0,
            secret_hash: first_secret_hash,
            issued_at: now,
            idle_expires_at: clamp_idle(now, idle_ttl, absolute_expires_at),
            status: TokenStatus::Issued,
        };
        (session, token)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        id: SessionId,
        user_id: UserId,
        created_at: DateTime<Utc>,
        absolute_expires_at: DateTime<Utc>,
        state: SessionState,
    ) -> Self {
        Self {
            id,
            user_id,
            created_at,
            absolute_expires_at,
            state,
        }
    }

    pub fn id(&self) -> SessionId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn absolute_expires_at(&self) -> &DateTime<Utc> {
        &self.absolute_expires_at
    }

    pub fn state(&self) -> &SessionState {
        &self.state
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, SessionState::Active)
    }

    /// 絶対期限内か。
    pub fn is_within_absolute<C: Clock>(&self, clock: &C) -> bool {
        clock.now() < self.absolute_expires_at
    }

    /// `Active` かつ絶対期限内なら生存。
    pub fn is_live<C: Clock>(&self, clock: &C) -> bool {
        self.is_active() && self.is_within_absolute(clock)
    }

    /// ファミリを失効させる。
    pub fn revoke<C: Clock>(&mut self, reason: RevocationReason, clock: &C) {
        if self.is_active() {
            self.state = SessionState::Revoked {
                at: clock.now(),
                reason,
            };
        }
    }
}

impl SessionToken {
    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        id: TokenId,
        session_id: SessionId,
        generation: u32,
        secret_hash: String,
        issued_at: DateTime<Utc>,
        idle_expires_at: DateTime<Utc>,
        status: TokenStatus,
    ) -> Self {
        Self {
            id,
            session_id,
            generation,
            secret_hash,
            issued_at,
            idle_expires_at,
            status,
        }
    }

    /// 後継世代を生成する。`generation + 1`・新しいアイドル期限(絶対期限でクランプ)・`Issued`。
    /// 旧世代の `Rotated` 化は repo の原子 CAS(`rotate_in`)で行う。
    pub fn rotate<C: Clock>(
        &self,
        new_token_id: TokenId,
        new_secret_hash: String,
        idle_ttl: Duration,
        session_absolute_expires_at: DateTime<Utc>,
        clock: &C,
    ) -> SessionToken {
        let now = clock.now();
        SessionToken {
            id: new_token_id,
            session_id: self.session_id,
            generation: self.generation + 1,
            secret_hash: new_secret_hash,
            issued_at: now,
            idle_expires_at: clamp_idle(now, idle_ttl, session_absolute_expires_at),
            status: TokenStatus::Issued,
        }
    }

    /// 在メモリモデルで `Rotated` へ遷移させる(永続化は repo の CAS が担う)。
    pub fn mark_rotated(&mut self) -> Result<(), InvalidTransitionError> {
        if self.status != TokenStatus::Issued {
            return Err(InvalidTransitionError {});
        }
        self.status = TokenStatus::Rotated;
        Ok(())
    }

    pub fn id(&self) -> TokenId {
        self.id
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn secret_hash(&self) -> &str {
        &self.secret_hash
    }

    pub fn issued_at(&self) -> &DateTime<Utc> {
        &self.issued_at
    }

    pub fn idle_expires_at(&self) -> &DateTime<Utc> {
        &self.idle_expires_at
    }

    pub fn status(&self) -> TokenStatus {
        self.status
    }

    /// `Issued` かつアイドル期限内なら回転に使える。
    pub fn is_live<C: Clock>(&self, clock: &C) -> bool {
        self.status == TokenStatus::Issued && clock.now() < self.idle_expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use uuid::Uuid;

    struct MockClock {
        now: DateTime<Utc>,
    }
    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    fn ids() -> (SessionId, TokenId) {
        (SessionId::new(Uuid::new_v4()), TokenId::new(Uuid::new_v4()))
    }

    #[test]
    fn test_start_creates_active_session_and_gen0_token() {
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let (sid, tid) = ids();
        let (session, token) = Session::start(
            sid,
            tid,
            UserId::new(Uuid::new_v4()),
            "hash0".to_string(),
            Duration::days(180),
            Duration::days(14),
            &MockClock { now: t0 },
        );
        assert!(session.is_active());
        assert_eq!(token.generation(), 0);
        assert_eq!(token.status(), TokenStatus::Issued);
        assert_eq!(token.session_id(), session.id());
        assert!(token.is_live(&MockClock { now: t0 }));
    }

    #[test]
    fn test_idle_is_clamped_to_absolute() {
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let (sid, tid) = ids();
        // idle_ttl(30d) > absolute_ttl(7d) なら idle は絶対期限でクランプされる。
        let (session, token) = Session::start(
            sid,
            tid,
            UserId::new(Uuid::new_v4()),
            "hash0".to_string(),
            Duration::days(7),
            Duration::days(30),
            &MockClock { now: t0 },
        );
        assert_eq!(token.idle_expires_at(), session.absolute_expires_at());
    }

    #[test]
    fn test_rotate_increments_generation_and_refreshes_idle() {
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let (sid, tid) = ids();
        let (session, token0) = Session::start(
            sid,
            tid,
            UserId::new(Uuid::new_v4()),
            "hash0".to_string(),
            Duration::days(180),
            Duration::days(14),
            &MockClock { now: t0 },
        );
        let t1 = t0 + Duration::days(1);
        let token1 = token0.rotate(
            TokenId::new(Uuid::new_v4()),
            "hash1".to_string(),
            Duration::days(14),
            *session.absolute_expires_at(),
            &MockClock { now: t1 },
        );
        assert_eq!(token1.generation(), 1);
        assert_eq!(token1.status(), TokenStatus::Issued);
        assert_eq!(token1.idle_expires_at(), &(t1 + Duration::days(14)));
    }

    #[test]
    fn test_token_not_live_after_idle_or_rotation() {
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let (sid, tid) = ids();
        let (_session, mut token) = Session::start(
            sid,
            tid,
            UserId::new(Uuid::new_v4()),
            "hash0".to_string(),
            Duration::days(180),
            Duration::days(14),
            &MockClock { now: t0 },
        );
        // アイドル期限超過。
        assert!(!token.is_live(&MockClock {
            now: t0 + Duration::days(15)
        }));
        // Rotated にすると(期限内でも)生存しない。
        token.mark_rotated().unwrap();
        assert!(!token.is_live(&MockClock { now: t0 }));
        assert!(token.mark_rotated().is_err());
    }

    #[test]
    fn test_revoke_sets_state() {
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let (sid, tid) = ids();
        let (mut session, _t) = Session::start(
            sid,
            tid,
            UserId::new(Uuid::new_v4()),
            "hash0".to_string(),
            Duration::days(180),
            Duration::days(14),
            &MockClock { now: t0 },
        );
        let t1 = t0 + Duration::hours(1);
        session.revoke(RevocationReason::ReuseDetected, &MockClock { now: t1 });
        assert!(!session.is_active());
        assert_eq!(
            session.state(),
            &SessionState::Revoked {
                at: t1,
                reason: RevocationReason::ReuseDetected
            }
        );
    }
}
