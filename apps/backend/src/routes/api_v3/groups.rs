mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(
        handlers::get_groups,
        handlers::put_group,
        handlers::get_group,
        handlers::patch_group,
        handlers::delete_group
    ))
}
