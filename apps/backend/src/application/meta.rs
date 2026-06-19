use crate::application::authz;
use crate::application::error::ApplicationError;
use crate::application::ports::meta_fetcher::{MetaFetcher, PageMeta};
use crate::domain::actor_ctx::ActorContext;

/// 外部 URL のメタ情報取得ユースケース(管理画面のリンクプレビュー用)。
pub struct MetaApp<'a, MF: MetaFetcher> {
    meta_fetcher: &'a MF,
}

impl<'a, MF: MetaFetcher> MetaApp<'a, MF> {
    pub fn new(meta_fetcher: &'a MF) -> Self {
        Self { meta_fetcher }
    }

    /// 認可後、指定 URL のメタ情報(title / description)を取得する。
    /// legacy 同様に管理者専用。
    pub async fn get_meta(
        &self,
        actor_ctx: &ActorContext,
        url: &str,
    ) -> Result<PageMeta, ApplicationError> {
        if !authz::can_fetch_meta(actor_ctx) {
            return Err(ApplicationError::Unauthorized);
        }
        self.meta_fetcher.fetch(url).await
    }
}
