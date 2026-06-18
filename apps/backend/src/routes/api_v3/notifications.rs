mod dto;
mod handlers;

pub(super) use dto::NotificationRead;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new().routes(routes!(
        handlers::get_notifications,
        handlers::post_notification,
        handlers::get_notification,
        handlers::patch_notification,
        handlers::delete_notification
    ))
}
