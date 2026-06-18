mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(
        handlers::get_users,
        handlers::put_user,
        handlers::get_user,
        handlers::patch_user,
        handlers::delete_user,
        handlers::get_user_notifications,
        handlers::post_user_m_address
    ))
}
