use crate::application::ports::clock::Clock;
use crate::domain::email_address::EmailAddress;
use crate::domain::error::InvalidTransitionError;
use crate::domain::one_time_token_id::OneTimeTokenId;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Duration, Utc};

/// ワンタイムトークンの用途。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OneTimeTokenPurpose {
    /// 初回有効化(activation)。`m_address` 束縛を伴う。
    Activation,
    /// パスワードリセット。
    PasswordReset,
}

impl OneTimeTokenPurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            OneTimeTokenPurpose::Activation => "activation",
            OneTimeTokenPurpose::PasswordReset => "password_reset",
        }
    }

    // as_str と対になる `&str -> Option<Self>` パーサ。FromStr(Result 返し)とは別物。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "activation" => Some(OneTimeTokenPurpose::Activation),
            "password_reset" => Some(OneTimeTokenPurpose::PasswordReset),
            _ => None,
        }
    }
}

/// 単一使用のワンタイムトークン(有効化・パスワードリセット)。
///
/// 平文トークンは `"{id}.{base64url(secret)}"` の形でユーザーへ配送し，DB には
/// `token_hash`(= HMAC-SHA256(pepper, secret) の hex。計算は infra 層)だけを保存する。
/// `id` は秘密ではなく単なるルックアップキーで，照合は `secret` の HMAC を定数時間比較する。
/// 単一使用は `consume`(在メモリ)と repo の条件付き UPDATE(consumed_at IS NULL)で原子的に担保する。
#[derive(Debug, Clone, PartialEq)]
pub struct OneTimeToken {
    id: OneTimeTokenId,
    user_id: UserId,
    purpose: OneTimeTokenPurpose,
    token_hash: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    consumed_at: Option<DateTime<Utc>>,
    m_address: Option<EmailAddress>,
}

impl OneTimeToken {
    /// 新規発行。`expires_at = now + ttl`。
    pub fn issue<C: Clock>(
        id: OneTimeTokenId,
        user_id: UserId,
        purpose: OneTimeTokenPurpose,
        token_hash: String,
        m_address: Option<EmailAddress>,
        ttl: Duration,
        clock: &C,
    ) -> Self {
        let now = clock.now();
        Self {
            id,
            user_id,
            purpose,
            token_hash,
            created_at: now,
            expires_at: now + ttl,
            consumed_at: None,
            m_address,
        }
    }

    /// DB などからの復元用。
    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        id: OneTimeTokenId,
        user_id: UserId,
        purpose: OneTimeTokenPurpose,
        token_hash: String,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        consumed_at: Option<DateTime<Utc>>,
        m_address: Option<EmailAddress>,
    ) -> Self {
        Self {
            id,
            user_id,
            purpose,
            token_hash,
            created_at,
            expires_at,
            consumed_at,
            m_address,
        }
    }

    pub fn id(&self) -> OneTimeTokenId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn purpose(&self) -> OneTimeTokenPurpose {
        self.purpose
    }

    pub fn token_hash(&self) -> &str {
        &self.token_hash
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn expires_at(&self) -> &DateTime<Utc> {
        &self.expires_at
    }

    pub fn consumed_at(&self) -> Option<&DateTime<Utc>> {
        self.consumed_at.as_ref()
    }

    pub fn m_address(&self) -> Option<&EmailAddress> {
        self.m_address.as_ref()
    }

    /// 未消費かつ未失効なら消費可能。
    pub fn is_consumable<C: Clock>(&self, clock: &C) -> bool {
        self.consumed_at.is_none() && clock.now() < self.expires_at
    }

    /// トークンを消費する。消費不可(消費済み/失効)の場合は遷移エラー。
    pub fn consume<C: Clock>(&mut self, clock: &C) -> Result<(), InvalidTransitionError> {
        if !self.is_consumable(clock) {
            return Err(InvalidTransitionError {});
        }
        self.consumed_at = Some(clock.now());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    struct MockClock {
        now: DateTime<Utc>,
    }
    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    fn token_at(now: DateTime<Utc>, ttl_secs: i64) -> OneTimeToken {
        OneTimeToken::issue(
            OneTimeTokenId::new(uuid::Uuid::new_v4()),
            UserId::new(uuid::Uuid::new_v4()),
            OneTimeTokenPurpose::Activation,
            "deadbeef".to_string(),
            Some(EmailAddress::new("a@example.com".to_string()).unwrap()),
            Duration::seconds(ttl_secs),
            &MockClock { now },
        )
    }

    #[test]
    fn test_consumable_until_consumed() {
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let mut token = token_at(t0, 3600);
        let clock = MockClock { now: t0 };
        assert!(token.is_consumable(&clock));
        token.consume(&clock).unwrap();
        assert!(!token.is_consumable(&clock));
        // 二度目の消費は不可。
        assert!(token.consume(&clock).is_err());
    }

    #[test]
    fn test_not_consumable_after_expiry() {
        let t0 = Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap();
        let token = token_at(t0, 60);
        let later = MockClock {
            now: t0 + Duration::seconds(61),
        };
        assert!(!token.is_consumable(&later));
    }

    #[test]
    fn test_purpose_str_roundtrip() {
        for p in [
            OneTimeTokenPurpose::Activation,
            OneTimeTokenPurpose::PasswordReset,
        ] {
            assert_eq!(OneTimeTokenPurpose::from_str(p.as_str()), Some(p));
        }
        assert_eq!(OneTimeTokenPurpose::from_str("nope"), None);
    }
}
