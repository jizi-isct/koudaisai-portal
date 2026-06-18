mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(handlers::get_meta))
}
