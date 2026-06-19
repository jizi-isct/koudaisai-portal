mod dto;
mod handlers;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    // `routes!` は同一パスのハンドラ(メソッド違い)をまとめる。異なるパスは別 `.routes()` に分ける。
    OpenApiRouter::new()
        .routes(routes!(handlers::get_users, handlers::post_user))
        .routes(routes!(
            handlers::get_user,
            handlers::patch_user,
            handlers::delete_user
        ))
        .routes(routes!(handlers::get_user_read_notifications))
        .routes(routes!(handlers::post_user_m_address))
}
