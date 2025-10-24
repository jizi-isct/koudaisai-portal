use crate::config::Secrets;
use axum::routing::{any_service, MethodRouter};
use axum_proxy::AppendPrefix;
use http::header::HOST;
use http::{HeaderName, HeaderValue};
use std::str::FromStr;
use axum::extract::Request;
use tower::ServiceBuilder;
use tower_http::set_header::SetRequestHeaderLayer;
use tracing::instrument;
use crate::middlewares::CurrentUser;
use crate::util::layers::RequireUserLayer;

#[instrument(name = "init /api/v2/users")]
pub fn init_service<S: Clone + 'static, E: From<std::convert::Infallible> + 'static>(
    secrets: Secrets,
) -> MethodRouter<S, E> {
    // 上流ホスト（:authority）を指定。
    let upstream = axum_proxy::builder_https("api2025.jizi.jp").expect("build proxy client");

    // /api/plans_info/{*path} の先頭1回だけ /v1 に置換
    // 例: /api/plans_info/plans?x=1 → /v1/plans?x=1
    let host = HeaderValue::from_static("api2025.jizi.jp");
    let cf_id =
        HeaderValue::from_str(secrets.plans_info_api_client_id.as_ref()).expect("valid header");
    let cf_secret =
        HeaderValue::from_str(secrets.plans_info_api_client_secret.as_ref()).expect("valid header");

    any_service(upstream.build(AppendPrefix("/v1"))).layer(
        ServiceBuilder::new()
            .layer(RequireUserLayer::admin_only())
            .layer(SetRequestHeaderLayer::overriding(HOST, host.clone()))
            .layer(SetRequestHeaderLayer::overriding(
                HeaderName::from_str("CF-Access-Client-Id").unwrap(),
                cf_id,
            ))
            .layer(SetRequestHeaderLayer::overriding(
                HeaderName::from_str("CF-Access-Client-Secret").unwrap(),
                cf_secret,
            )),
    )
}
