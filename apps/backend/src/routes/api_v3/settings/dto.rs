use crate::domain::settings::Settings;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 管理者向けのグローバル設定全体。
#[derive(Serialize, ToSchema)]
pub struct SettingsRead {
    show_occasions_on_portal: bool,
}

impl From<Settings> for SettingsRead {
    fn from(settings: Settings) -> Self {
        Self {
            show_occasions_on_portal: settings.show_occasions_on_portal(),
        }
    }
}

/// 参加団体へ公開してよい企画実施場所の表示設定。
#[derive(Serialize, ToSchema)]
pub struct ShowOccasionsOnPortalRead {
    show_occasions_on_portal: bool,
}

impl From<bool> for ShowOccasionsOnPortalRead {
    fn from(show_occasions_on_portal: bool) -> Self {
        Self {
            show_occasions_on_portal,
        }
    }
}

/// 企画実施場所の表示設定を変更するリクエスト。
#[derive(Deserialize, ToSchema)]
pub struct ShowOccasionsOnPortalUpdate {
    pub show_occasions_on_portal: bool,
}
