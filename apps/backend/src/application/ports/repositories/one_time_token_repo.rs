use crate::application::error::FindError;
use crate::application::transaction::Transaction;
use crate::domain::one_time_token::{OneTimeToken, OneTimeTokenPurpose};
use crate::domain::one_time_token_id::OneTimeTokenId;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};

#[async_trait::async_trait]
pub trait OneTimeTokenRepo<Tx: Transaction> {
    async fn find_by_id(&self, id: OneTimeTokenId) -> Result<Option<OneTimeToken>, FindError>;

    async fn insert_in(&self, tx: &mut Tx, token: &OneTimeToken) -> Result<(), anyhow::Error>;

    /// 単一使用の原子的消費。`consumed_at IS NULL` かつ未失効(`expires_at > now`)の行だけを
    /// `now` で消費する。消費できたら 1、既に消費済み/失効/不在なら 0 を返す。
    async fn consume_in(
        &self,
        tx: &mut Tx,
        id: OneTimeTokenId,
        now: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error>;

    /// 同一 `user_id` × `purpose` の未消費トークンを一括で無効化(消費)する。
    /// トークン再発行時に既存の有効トークンを失効させる用途。失効させた件数を返す。
    async fn invalidate_existing_for_in(
        &self,
        tx: &mut Tx,
        user_id: UserId,
        purpose: OneTimeTokenPurpose,
        now: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error>;

    /// 失効済みトークンを掃除する。削除件数を返す。
    async fn delete_expired(&self, now: DateTime<Utc>) -> Result<u64, anyhow::Error>;
}
