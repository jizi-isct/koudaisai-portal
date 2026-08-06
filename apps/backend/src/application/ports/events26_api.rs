use crate::application::error::{DeleteError, InsertError, OperationError, UpdateError};
use events26_api::models::Project;
use thiserror::Error;

/// 企画アイコンの更新に失敗した理由。
///
/// 画像の検証(空でないか・20MB 以下か・対応形式か・正方形か)は events26 側が行う。
/// ポータルは中身を見ないので、弾かれた理由は [`InvalidImage`](Self::InvalidImage) に
/// 文字列で載せて呼び出し側へ返す。[`UpdateError`] に相当する枠が無いため専用にした。
#[derive(Debug, Error)]
pub enum UpdateIconError {
    #[error("Not found")]
    NotFound,
    #[error("Invalid image: {0}")]
    InvalidImage(String),
    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}

impl OperationError for UpdateIconError {}

/// 企画情報API(events26)の `/admin/v1` 配下への操作を表すポート。
///
/// リクエスト/レスポンスの型は OpenAPI 仕様から生成した `events26_api::models` を
/// そのまま用いる。ドメインの型へ写すと仕様変更のたびに二重にメンテすることになり、
/// このポートは外部 API の薄いゲートウェイに徹する方が保守しやすいため。
///
/// base URL と Cloudflare Access のサービストークン(`CF-Access-Client-Id` /
/// `CF-Access-Client-Secret`)の設定は実装側の責務とする。
#[async_trait::async_trait]
pub trait Events26Api {
    /// 企画を新規登録する。ID は呼び出し側が [`Project`] の中で指定する。
    /// 同じ ID が既に存在する場合は [`InsertError::Conflict`]。
    async fn create_project(&self, project: &Project) -> Result<Project, InsertError>;

    /// 企画を丸ごと置き換える。タグと開催予定は差分ではなく総入れ替えになる。
    /// 指定 ID の企画が無い場合は [`UpdateError::NotFound`]。
    async fn update_project(
        &self,
        project_id: &str,
        project: &Project,
    ) -> Result<Project, UpdateError>;

    /// 企画を削除する。タグと開催予定も一緒に消える。
    /// 指定 ID の企画が無い場合は [`DeleteError::NotFound`]。
    async fn delete_project(&self, project_id: &str) -> Result<(), DeleteError>;

    /// 企画アイコンの原本を差し替える。`content_type` は `image/png` などの
    /// メディアタイプで、そのまま `Content-Type` として送る(events26 が形式判定に使う)。
    async fn update_project_icon(
        &self,
        project_id: &str,
        content_type: &str,
        image: Vec<u8>,
    ) -> Result<(), UpdateIconError>;

    /// 企画アイコンの原本を削除する。未登録でも成功する。
    async fn delete_project_icon(&self, project_id: &str) -> Result<(), DeleteError>;
}
