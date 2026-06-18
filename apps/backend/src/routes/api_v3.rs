mod groups;
mod notifications;
mod users;

use utoipa_axum::router::OpenApiRouter;

const GROUPS_TAG: &str = "groups";
const NOTIFICATIONS_TAG: &str = "notifications";
const USERS_TAG: &str = "users";

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/users", users::router())
        .nest("/groups", groups::router())
        .nest("/notifications", notifications::router())
}
