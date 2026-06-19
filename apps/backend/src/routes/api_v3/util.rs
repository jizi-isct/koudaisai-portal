mod dto;
mod handlers;

pub(super) use dto::MetaInfo;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new().routes(routes!(handlers::get_meta))
}
