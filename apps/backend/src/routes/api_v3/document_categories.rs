mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(
        handlers::get_document_categories,
        handlers::get_document_category,
        handlers::post_document_category,
        handlers::patch_document_category,
        handlers::delete_document_category
    ))
}
