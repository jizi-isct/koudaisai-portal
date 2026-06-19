mod dto;
mod handlers;

pub(super) use dto::DocumentCategoryRead;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new()
        .routes(routes!(
            handlers::get_document_categories,
            handlers::post_document_category
        ))
        .routes(routes!(
            handlers::get_document_category,
            handlers::patch_document_category,
            handlers::delete_document_category
        ))
}
