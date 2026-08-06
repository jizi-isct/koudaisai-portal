mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new()
        .routes(routes!(handlers::post_project))
        .routes(routes!(handlers::put_project, handlers::delete_project))
}
