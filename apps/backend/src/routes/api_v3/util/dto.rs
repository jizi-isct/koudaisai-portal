use crate::application::ports::meta_fetcher::PageMeta;
use serde::Serialize;
use utoipa::ToSchema;

/// 取得先 URL の OpenGraph / HTML メタ情報（リンクプレビュー用）。
#[derive(Serialize, ToSchema)]
pub struct MetaInfo {
    title: Option<String>,
    description: Option<String>,
}

impl From<PageMeta> for MetaInfo {
    fn from(pm: PageMeta) -> Self {
        MetaInfo {
            title: pm.title,
            description: pm.description,
        }
    }
}
