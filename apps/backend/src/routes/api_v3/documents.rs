mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(
        handlers::get_documents,
        handlers::post_document,
        handlers::get_document,
        handlers::patch_document,
        handlers::delete_document
    ))
}
