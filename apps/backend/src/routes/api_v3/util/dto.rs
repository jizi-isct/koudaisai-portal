use serde::Serialize;
use utoipa::ToSchema;

/// 取得先 URL の OpenGraph / HTML メタ情報（リンクプレビュー用）。
#[derive(Serialize, ToSchema)]
pub struct MetaInfo {
    title: Option<String>,
    description: Option<String>,
}
