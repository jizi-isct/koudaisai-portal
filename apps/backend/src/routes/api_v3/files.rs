mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(
        handlers::post_file_upload,
        handlers::get_file_download
    ))
}
