mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new()
        .routes(routes!(handlers::get_documents, handlers::post_document))
        .routes(routes!(handlers::get_documents_by_category))
        .routes(routes!(
            handlers::get_document,
            handlers::patch_document,
            handlers::delete_document
        ))
}
