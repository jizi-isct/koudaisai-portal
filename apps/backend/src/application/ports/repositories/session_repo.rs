use crate::application::error::FindError;
use crate::application::transaction::Transaction;
use crate::domain::session::{RevocationReason, Session, SessionToken};
use crate::domain::session_id::SessionId;
use crate::domain::token_id::TokenId;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};

/// 回転セッションの永続化ポート。1 リポジトリで `sessions`(ファミリ) と
/// `session_tokens`(世代) の2テーブルを扱う。原子回転は `rotate_in` の CAS で担保する。
#[async_trait::async_trait]
pub trait SessionRepo<Tx: Transaction> {
    async fn find_session_by_id(&self, id: SessionId) -> Result<Option<Session>, FindError>;

    async fn find_token_by_id(&self, id: TokenId) -> Result<Option<SessionToken>, FindError>;

    /// `Active` なファミリ一覧(LRU 退避・件数確認に用いる)。
    async fn find_active_sessions_by_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<Session>, FindError>;

    /// ファミリと第0世代トークンを挿入する。
    async fn insert_session_in(
        &self,
        tx: &mut Tx,
        session: &Session,
        first: &SessionToken,
    ) -> Result<(), anyhow::Error>;

    /// 原子回転。`old_token_id` の行を `status='issued'` の条件付きで `rotated` にできた場合だけ
    /// 後継世代 `new` を挿入する。回転できたら 1、既に回転/失効済み(=並行回転や reuse)なら 0 を返す。
    async fn rotate_in(
        &self,
        tx: &mut Tx,
        old_token_id: TokenId,
        new: &SessionToken,
    ) -> Result<u64, anyhow::Error>;

    /// ファミリ全体を失効させる(セッションと配下の全世代トークン)。
    async fn revoke_family_in(
        &self,
        tx: &mut Tx,
        session_id: SessionId,
        reason: RevocationReason,
        at: DateTime<Utc>,
    ) -> Result<(), anyhow::Error>;

    /// あるユーザーの全 `Active` ファミリを失効させる。失効件数を返す。
    /// 集約ロードを介さない set ベースの意図的な例外。`at` はユースケースが `Clock` から渡す。
    async fn revoke_all_for_user_in(
        &self,
        tx: &mut Tx,
        user_id: UserId,
        reason: RevocationReason,
        at: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error>;

    /// 失効済み/絶対期限切れのファミリ配下のみ掃除する。
    /// **`Active` ファミリの `Rotated` 行は消さない**(消すと reuse 検知に盲点が生じる)。
    async fn delete_expired(&self, now: DateTime<Utc>) -> Result<u64, anyhow::Error>;
}
