use crate::domain::settings::Settings;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 管理者向けのグローバル設定全体。
#[derive(Serialize, ToSchema)]
pub struct SettingsRead {
    show_occasions_on_portal: bool,
    accept_correction_requests: bool,
}

impl From<Settings> for SettingsRead {
    fn from(settings: Settings) -> Self {
        Self {
            show_occasions_on_portal: settings.show_occasions_on_portal(),
            accept_correction_requests: settings.accept_correction_requests(),
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

/// 参加団体が企画情報の訂正申請を出せるかどうか。
#[derive(Serialize, ToSchema)]
pub struct AcceptCorrectionRequestsRead {
    accept_correction_requests: bool,
}

impl From<bool> for AcceptCorrectionRequestsRead {
    fn from(accept_correction_requests: bool) -> Self {
        Self {
            accept_correction_requests,
        }
    }
}

/// 訂正申請の受付状態を変更するリクエスト。
#[derive(Deserialize, ToSchema)]
pub struct AcceptCorrectionRequestsUpdate {
    pub accept_correction_requests: bool,
}
