mod dto;
mod handlers;

pub(super) use dto::{SettingsRead, ShowOccasionsOnPortalRead, ShowOccasionsOnPortalUpdate};

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new()
        .routes(routes!(handlers::get_settings))
        .routes(routes!(
            handlers::get_show_occasions_on_portal,
            handlers::patch_show_occasions_on_portal
        ))
}
