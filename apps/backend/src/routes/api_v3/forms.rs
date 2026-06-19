mod dto;
mod handlers;

pub(super) use dto::FormRead;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new()
        .routes(routes!(handlers::get_forms, handlers::post_form))
        .routes(routes!(
            handlers::get_form,
            handlers::patch_form,
            handlers::delete_form
        ))
}
