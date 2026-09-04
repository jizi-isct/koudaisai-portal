mod handlers;

use axum::extract::DefaultBodyLimit;
use utoipa_axum::router::{OpenApiRouter, UtoipaMethodRouterExt};
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    OpenApiRouter::new()
        .routes(routes!(handlers::post_project))
        .routes(routes!(handlers::put_project, handlers::delete_project))
        .routes(routes!(
            handlers::put_own_project_menu,
            handlers::delete_own_project_menu
        ))
        // アイコンは画像本体を運ぶので、axum 既定の 2MB では足りない。
        // events26 の上限に合わせてこのルートだけ引き上げる。
        .routes(
            routes!(handlers::put_project_icon, handlers::delete_project_icon)
                .layer(DefaultBodyLimit::max(handlers::ICON_MAX_BYTES)),
        )
}
