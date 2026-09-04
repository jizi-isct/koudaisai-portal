use crate::application::authz;
use crate::application::error::{ApplicationOperationError, DeleteError, InsertError, UpdateError};
use crate::application::ports::events26_api::{Events26Api, UpdateIconError, UpdateMenuError};
use crate::domain::actor_ctx::ActorContext;
use events26_api::models::{GetProjectDetails200ResponseMenu, Project};

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

    /// 認可後、企画アイコンの原本を差し替える。
    ///
    /// アイコンは企画の一部なので、専用の権限は設けず企画の更新権限で判定する。
    /// 画像そのものの検証(形式・サイズ・縦横比)は events26 側に委ねる。
    pub async fn update_project_icon(
        &self,
        actor_ctx: &ActorContext,
        project_id: &str,
        content_type: &str,
        image: Vec<u8>,
    ) -> Result<(), ApplicationOperationError<UpdateIconError>> {
        if !authz::can_update_events26_project(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self
            .events26_api
            .update_project_icon(project_id, content_type, image)
            .await?)
    }

    /// 認可後、企画アイコンの原本を削除する。未登録でも成功する。
    /// 判定は [`Self::update_project_icon`] と同じく企画の更新権限で行う
    /// (企画そのものを消すわけではないため削除権限ではない)。
    pub async fn delete_project_icon(
        &self,
        actor_ctx: &ActorContext,
        project_id: &str,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        if !authz::can_update_events26_project(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self.events26_api.delete_project_icon(project_id).await?)
    }

    /// ログイン中の参加団体自身の企画メニューを保存する。
    /// 企画 ID は所属団体 ID から決定し、呼び出し側には指定させない。
    pub async fn update_own_project_menu(
        &self,
        actor_ctx: &ActorContext,
        menu: &GetProjectDetails200ResponseMenu,
    ) -> Result<(), ApplicationOperationError<UpdateMenuError>> {
        if !authz::can_update_own_events26_menu(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        let Some(project_id) = actor_ctx.primary_group_id() else {
            return Err(ApplicationOperationError::Unauthorized);
        };
        Ok(self
            .events26_api
            .update_project_menu(&project_id.to_string(), menu)
            .await?)
    }

    /// ログイン中の参加団体自身の企画メニューを削除する。
    pub async fn delete_own_project_menu(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        if !authz::can_update_own_events26_menu(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        let Some(project_id) = actor_ctx.primary_group_id() else {
            return Err(ApplicationOperationError::Unauthorized);
        };
        Ok(self
            .events26_api
            .delete_project_menu(&project_id.to_string())
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::group::GroupType;
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::{Membership, Role};
    use crate::domain::user_id::UserId;
    use crate::infra::memory::clock_impl::MemoryClock;
    use crate::infra::memory::events26_api_impl::MemoryEvents26Api;
    use chrono::{TimeZone, Utc};
    use std::str::FromStr;
    use uuid::Uuid;

    fn user_ctx(group_type: GroupType) -> ActorContext {
        let user_id = UserId::new(Uuid::new_v4());
        ActorContext::User {
            user_id,
            name: "参加者".to_string(),
            memberships: vec![Membership::new(
                GroupId::from_str("I-100").unwrap(),
                user_id,
                Role::FirstResponsible,
                &MemoryClock::new(Utc.timestamp_opt(0, 0).unwrap()),
            )],
            group_type,
        }
    }

    fn menu() -> GetProjectDetails200ResponseMenu {
        GetProjectDetails200ResponseMenu::new(vec![], "販売メニュー".to_string())
    }

    #[tokio::test]
    async fn participant_updates_and_deletes_own_group_menu() {
        let api = MemoryEvents26Api::new();
        let app = Events26App::new(&api);
        let actor = user_ctx(GroupType::GeneralProject);
        let menu = menu();

        app.update_own_project_menu(&actor, &menu).await.unwrap();
        assert_eq!(api.menu("I-100"), Some(menu));

        app.delete_own_project_menu(&actor).await.unwrap();
        assert_eq!(api.menu("I-100"), None);
    }

    #[tokio::test]
    async fn unauthenticated_user_cannot_update_menu() {
        let api = MemoryEvents26Api::new();
        let app = Events26App::new(&api);

        let result = app
            .update_own_project_menu(&ActorContext::NoLogin, &menu())
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
        assert_eq!(api.menu("I-100"), None);
    }

    #[tokio::test]
    async fn press_group_cannot_update_menu() {
        let api = MemoryEvents26Api::new();
        let app = Events26App::new(&api);

        let result = app
            .update_own_project_menu(&user_ctx(GroupType::Press), &menu())
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
        assert_eq!(api.menu("I-100"), None);
    }
}
