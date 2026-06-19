mod dto;
mod handlers;

pub(super) use dto::ApprovalRequestRead;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new()
        .routes(routes!(
            handlers::get_approval_requests,
            handlers::post_approval_request
        ))
        .routes(routes!(
            handlers::get_approval_request,
            handlers::delete_approval_request
        ))
        .routes(routes!(handlers::approve_approval_request))
        .routes(routes!(handlers::reject_approval_request))
        .routes(routes!(handlers::close_approval_request))
}
