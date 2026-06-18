mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new().routes(routes!(
        handlers::get_groups,
        handlers::put_group,
        handlers::get_group,
        handlers::patch_group,
        handlers::delete_group,
        handlers::get_members,
        handlers::put_member,
        handlers::delete_member
    ))
}
