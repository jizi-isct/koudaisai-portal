use crate::application::authz;
use crate::application::error::{ApplicationOperationError, DeleteError, InsertError, UpdateError};
use crate::application::ports::events26_api::Events26Api;
use crate::domain::actor_ctx::ActorContext;
use events26_api::models::Project;

/// 企画情報API(events26)の企画管理ユースケース。
///
/// 企画データの正本は外部の events26 API 側にあるため、リポジトリではなく
/// [`Events26Api`] ポート経由で操作する。ポータル側に写しを持たないので
/// トランザクションも張らない。
///
/// 企画の型は OpenAPI 仕様から生成した [`Project`] をそのまま受け渡す
/// ([`Events26Api`] と同じ理由による)。
pub struct Events26App<'a, EA: Events26Api> {
    events26_api: &'a EA,
}

impl<'a, EA: Events26Api> Events26App<'a, EA> {
    pub fn new(events26_api: &'a EA) -> Self {
        Self { events26_api }
    }

    /// 認可後、企画を新規登録する。ID は `project` の中で呼び出し側が指定する。
    /// 同じ ID が既にある場合は `InsertError::Conflict`。
    pub async fn create_project(
        &self,
        actor_ctx: &ActorContext,
        project: &Project,
    ) -> Result<Project, ApplicationOperationError<InsertError>> {
        if !authz::can_create_events26_project(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self.events26_api.create_project(project).await?)
    }

    /// 認可後、企画を丸ごと置き換える。タグと開催予定は差分ではなく総入れ替えになる。
    /// 指定 ID の企画が無い場合は `UpdateError::NotFound`。
    pub async fn update_project(
        &self,
        actor_ctx: &ActorContext,
        project_id: &str,
        project: &Project,
    ) -> Result<Project, ApplicationOperationError<UpdateError>> {
        if !authz::can_update_events26_project(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self
            .events26_api
            .update_project(project_id, project)
            .await?)
    }

    /// 認可後、企画を削除する。タグと開催予定も一緒に消える。
    /// 指定 ID の企画が無い場合は `DeleteError::NotFound`。
    pub async fn delete_project(
        &self,
        actor_ctx: &ActorContext,
        project_id: &str,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        if !authz::can_delete_events26_project(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self.events26_api.delete_project(project_id).await?)
    }
}
