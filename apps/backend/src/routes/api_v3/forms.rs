mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new().routes(routes!(
        handlers::get_forms,
        handlers::post_form,
        handlers::get_form,
        handlers::patch_form,
        handlers::delete_form
    ))
}
