use crate::application::authz;
use crate::application::error::{ApplicationOperationError, FindError, UpdateError};
use crate::application::ports::repositories::settings_repo::SettingsRepo;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::settings::Settings;

/// グローバル設定の参照・変更ユースケース。
pub struct SettingsApp<'a, STR: SettingsRepo> {
    settings_repo: &'a STR,
}

impl<'a, STR: SettingsRepo> SettingsApp<'a, STR> {
    pub fn new(settings_repo: &'a STR) -> Self {
        Self { settings_repo }
    }

    /// 管理者に現在の設定をすべて返す。
    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Settings, ApplicationOperationError<FindError>> {
        if !authz::can_get_all_settings(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self.settings_repo.get().await?)
    }

    /// 参加団体が企画実施場所を表示してよいかを取得する。
    pub async fn get_show_occasions_on_portal(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<bool, ApplicationOperationError<FindError>> {
        if !authz::can_get_show_occasions_on_portal(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self.settings_repo.get().await?.show_occasions_on_portal())
    }

    /// 企画実施場所を参加団体へ表示するかどうかを変更する。
    pub async fn change_show_occasions_on_portal(
        &self,
        actor_ctx: &ActorContext,
        enabled: bool,
    ) -> Result<Settings, ApplicationOperationError<UpdateError>> {
        if !authz::can_write_settings(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let mut settings = self.settings_repo.get().await.map_err(|e| {
            ApplicationOperationError::OperationFailed(UpdateError::InternalError(e.into()))
        })?;
        settings.change_show_occasions_on_portal(enabled);
        self.settings_repo.save(&settings).await?;
        Ok(settings)
    }
}
