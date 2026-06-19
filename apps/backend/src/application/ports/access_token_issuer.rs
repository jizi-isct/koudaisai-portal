//! 短命アクセストークン(JWT)の発行ポート。
//!
//! `AuthApp` を RSA 鍵なしでテストできるよう薄くポート化する(`Clock`/`Email` と同方針)。
//! 検証はミドルウェア側が行うため，ここは発行のみを担う。

use crate::domain::session_id::SessionId;
use crate::domain::user_id::UserId;
use chrono::Duration;

pub trait AccessTokenIssuer: Send + Sync {
    /// `user_id` 向けの短命アクセストークンを発行する。`sid` はセッション(ファミリ)id で，
    /// トークンに埋め込み，失効の即時反映や監査に用いる。
    fn issue(&self, user_id: UserId, sid: SessionId, ttl: Duration) -> anyhow::Result<String>;
}
