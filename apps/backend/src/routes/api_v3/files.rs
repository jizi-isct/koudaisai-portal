mod dto;
mod handlers;

pub(super) use dto::{FileDownloadResponse, FileUploadResponse};

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new()
        .routes(routes!(handlers::post_file_upload))
        .routes(routes!(handlers::get_file_download))
}
