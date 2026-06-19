//! 高エントロピー秘密・識別子の生成とハッシュ照合を担うポート。
//!
//! リフレッシュトークンの secret(256bit 一様乱数)やワンタイムトークンの secret，
//! および各種 id を払い出す。secret のハッシュは memory-hard である必要がなく
//! (総当たり不能なため)，HMAC-SHA256(pepper, secret) で前像耐性と
//! 「DB 単独流出時の再構成不能性」を確保する。照合は定数時間で行う。

use uuid::Uuid;

pub trait SecretGenerator: Send + Sync {
    /// 高エントロピー秘密(base64url 文字列)を生成する。
    fn generate_secret(&self) -> String;

    /// secret のハッシュ(HMAC-SHA256(pepper, secret) の hex)を計算する。保存用。
    fn hash_secret(&self, secret: &str) -> String;

    /// 提示された `secret` が保存済み `hash` と一致するかを定数時間で検証する。
    fn verify_secret(&self, secret: &str, hash: &str) -> bool;

    /// 新しい識別子(session_id / token_id / one_time_token_id 等)を払い出す。
    fn new_id(&self) -> Uuid;
}
