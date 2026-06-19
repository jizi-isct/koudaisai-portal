mod dto;
mod handlers;

pub(super) use dto::{GroupRead, MemberRead};

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<super::V3State> {
    // `routes!` は同一パスのハンドラ(メソッド違い)をまとめる。異なるパスは別 `.routes()` に分ける。
    OpenApiRouter::new()
        .routes(routes!(handlers::get_groups))
        .routes(routes!(
            handlers::put_group,
            handlers::get_group,
            handlers::patch_group,
            handlers::delete_group
        ))
        .routes(routes!(handlers::get_members))
        .routes(routes!(handlers::put_member, handlers::delete_member))
}
