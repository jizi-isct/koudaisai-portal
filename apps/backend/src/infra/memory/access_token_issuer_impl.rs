//! テスト用の決定的 [`AccessTokenIssuer`]。JWT を作らず文字列を組み立てるだけ。

use crate::application::ports::access_token_issuer::AccessTokenIssuer;
use crate::domain::session_id::SessionId;
use crate::domain::user_id::UserId;
use chrono::Duration;

#[derive(Default)]
pub struct MemoryAccessTokenIssuer;

impl MemoryAccessTokenIssuer {
    pub fn new() -> Self {
        Self
    }
}

impl AccessTokenIssuer for MemoryAccessTokenIssuer {
    fn issue(&self, user_id: UserId, sid: SessionId, ttl: Duration) -> anyhow::Result<String> {
        Ok(format!("access:{user_id}:{sid}:{}", ttl.num_seconds()))
    }
}
