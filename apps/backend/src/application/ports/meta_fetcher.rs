use crate::application::error::ApplicationError;

/// 取得先 URL の OpenGraph / HTML メタ情報(リンクプレビュー用)。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PageMeta {
    pub title: Option<String>,
    pub description: Option<String>,
}

/// 外部 URL の HTML を取得してメタ情報(title / description)を抽出するゲートウェイ。
/// 永続化集約ではなく外部サービス抽象(email / object_storage と同種)としてポート化する。
#[async_trait::async_trait]
pub trait MetaFetcher {
    /// 指定 URL の HTML を取得し、OpenGraph / `<title>` / meta description から
    /// メタ情報を抽出する。取得・解析に失敗した場合は `InternalError`。
    async fn fetch(&self, url: &str) -> Result<PageMeta, ApplicationError>;
}
