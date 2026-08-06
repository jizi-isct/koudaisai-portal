use crate::application::error::{DeleteError, InsertError, UpdateError};
use events26_api::models::Project;

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
}
