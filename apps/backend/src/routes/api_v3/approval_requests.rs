mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new().routes(routes!(
        handlers::get_approval_requests,
        handlers::post_approval_request,
        handlers::get_approval_request,
        handlers::approve_approval_request,
        handlers::reject_approval_request,
        handlers::close_approval_request,
        handlers::delete_approval_request
    ))
}
