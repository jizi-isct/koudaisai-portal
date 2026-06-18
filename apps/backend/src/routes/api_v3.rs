mod approval_requests;
mod document_categories;
mod documents;
mod files;
mod forms;
mod groups;
mod notifications;
mod users;
mod util;

use utoipa_axum::router::OpenApiRouter;

const APPROVAL_REQUESTS_TAG: &str = "approval-requests";
const DOCUMENTS_TAG: &str = "documents";
const DOCUMENT_CATEGORIES_TAG: &str = "document-categories";
const FILES_TAG: &str = "files";
const FORMS_TAG: &str = "forms";
const GROUPS_TAG: &str = "groups";
const NOTIFICATIONS_TAG: &str = "notifications";
const USERS_TAG: &str = "users";
const UTIL_TAG: &str = "util";

/// api_v3 全体で共有する State。auth_v2 と同じ `AuthV2State` を使う
/// (`Actor` 抽出子が `FromRequestParts<AuthV2State>` のため)。
pub(crate) type V3State = crate::routes::auth_v2::AuthV2State;

pub fn router() -> OpenApiRouter<V3State> {
    OpenApiRouter::new()
        .nest("/users", users::router())
        .nest("/groups", groups::router())
        .nest("/notifications", notifications::router())
        .nest("/forms", forms::router())
        .nest("/document-categories", document_categories::router())
        .nest("/documents", documents::router())
        .nest("/approval-requests", approval_requests::router())
        .nest("/files", files::router())
        .nest("/util", util::router())
}
