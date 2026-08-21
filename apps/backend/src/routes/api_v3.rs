mod approval_requests;
mod document_categories;
mod documents;
mod events26;
mod files;
mod forms;
mod groups;
mod notifications;
mod settings;
mod users;
mod util;

use utoipa_axum::router::OpenApiRouter;

const APPROVAL_REQUESTS_TAG: &str = "approval-requests";
const DOCUMENTS_TAG: &str = "documents";
const DOCUMENT_CATEGORIES_TAG: &str = "document-categories";
const EVENTS26_TAG: &str = "events26";
const FILES_TAG: &str = "files";
const FORMS_TAG: &str = "forms";
const GROUPS_TAG: &str = "groups";
const NOTIFICATIONS_TAG: &str = "notifications";
const SETTINGS_TAG: &str = "settings";
const USERS_TAG: &str = "users";
const UTIL_TAG: &str = "util";

/// api_v3 全体で共有する State。auth_v2 と同じ `AuthV2State` を使う
/// (`ActorContext` 抽出子が `FromRequestParts<AuthV2State>` のため)。
pub(crate) type V3State = crate::routes::auth_v2::AuthV2State;

/// レスポンス DTO のスキーマ登録用ベースドキュメント。
/// `#[http_response]` マクロは `$ref` を吐くが components へスキーマを登録しないため、
/// レスポンス系 DTO をここで明示的に列挙して OpenApi の components に載せる
/// (utoipa が入れ子の依存スキーマも再帰的に収集する)。
#[derive(utoipa::OpenApi)]
#[openapi(components(schemas(
    groups::GroupRead,
    groups::MemberRead,
    users::UserRead,
    users::UserCreated,
    users::MAddressUpdated,
    forms::FormRead,
    documents::DocumentRead,
    documents::DocumentsByCategoryEntry,
    document_categories::DocumentCategoryRead,
    notifications::NotificationRead,
    approval_requests::ApprovalRequestRead,
    files::FileUploadResponse,
    files::FileDownloadResponse,
    util::MetaInfo,
    events26_api::models::Project,
    settings::SettingsRead,
    settings::ShowOccasionsOnPortalRead,
    settings::ShowOccasionsOnPortalUpdate,
    settings::AcceptCorrectionRequestsRead,
    settings::AcceptCorrectionRequestsUpdate,
)))]
struct ApiV3Schemas;

pub fn router() -> OpenApiRouter<V3State> {
    OpenApiRouter::with_openapi(<ApiV3Schemas as utoipa::OpenApi>::openapi())
        .nest("/users", users::router())
        .nest("/groups", groups::router())
        .nest("/notifications", notifications::router())
        .nest("/forms", forms::router())
        .nest("/document-categories", document_categories::router())
        .nest("/documents", documents::router())
        .nest("/approval-requests", approval_requests::router())
        .nest("/files", files::router())
        .nest("/events26", events26::router())
        .nest("/settings", settings::router())
        .nest("/util", util::router())
}

#[cfg(test)]
mod tests {
    /// 全 api_v3 サブルータ + auth_v2 を main.rs と同じ形で nest し、
    /// ルート/nest の衝突(axum はビルド時に panic する)が無いことを確認する。
    #[test]
    fn v3_routers_nest_without_conflict() {
        let (auth_v2, _) = crate::routes::auth_v2::router().split_for_parts();
        let (api_v3, _) = super::router().split_for_parts();
        let _app: axum::Router<crate::routes::auth_v2::AuthV2State> = axum::Router::new()
            .nest("/auth/v2", auth_v2)
            .nest("/api/v3", api_v3);
    }
}
