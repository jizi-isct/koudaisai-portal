use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

/// セッショントークン(=世代ごとのリフレッシュトークン行)の識別子。
/// 不透明リフレッシュトークン `"{token_id}.{secret}"` のルックアップキーに使う。
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
pub struct TokenId(Uuid);

impl TokenId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl Display for TokenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<TokenId> for Uuid {
    fn from(id: TokenId) -> Self {
        id.0
    }
}
