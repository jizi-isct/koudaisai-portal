use axum::body::Body;
use axum::extract::Request;
use axum_proxy::client::{HttpConnector, HttpsConnector};
use axum_proxy::{AppendPrefix, ReusedService};
use tower::util::MapRequest;
use tower::ServiceExt;
use tracing::instrument;

#[instrument(name = "init /api/v2/users")]
pub fn init_service<'a>() -> MapRequest<
    ReusedService<AppendPrefix<'a>, HttpsConnector<HttpConnector>, Body>,
    fn(Request) -> Request<Body>,
> {
    // 上流ホスト（:authority）を指定。
    let upstream = axum_proxy::builder_https("api2025.jizi.jp").expect("build proxy client");

    // /api/plans_info/{*path} の先頭1回だけ /v1 に置換
    // 例: /api/plans_info/plans?x=1 → /v1/plans?x=1
    upstream
        .build(AppendPrefix("/v1"))
        .map_request(|mut req: Request<Body>| {
            req.headers_mut()
                .insert("host", "api2025.jizi.jp".parse().unwrap());
            req
        })
}
