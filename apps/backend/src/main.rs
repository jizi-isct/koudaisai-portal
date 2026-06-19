// TODO(sqlx移行): Db 設定型は init_db 復活時に再度 import する
use crate::application::auth::AuthConfig;
use crate::application::ports::password_hasher::PasswordHasher;
use crate::application::ports::secret_generator::SecretGenerator;
use crate::config::{Logging, init_config};
use crate::domain::plaintext_password::PlaintextPassword;
use crate::infra::argon2_password_hasher::Argon2PasswordHasher;
use crate::infra::discord_webhook::WebhookDiscord;
use crate::infra::jwt_access_token_issuer::JwtAccessTokenIssuer;
use crate::infra::random_secret_generator::RandomSecretGenerator;
use crate::infra::s3_object_storage::S3ObjectStorage;
use crate::infra::sendgrid_email::SendgridEmail;
use crate::infra::sqlite::{connect_and_migrate, new_sqlite_application};
use crate::routes::auth_v2::{AuthV2State, CookieConfig};
use chrono::Duration;
use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub(crate) mod application;
pub mod config;
pub(crate) mod domain;
// TODO(sqlx移行): entities/sea_orm_entities 層は撤去。DBアクセスは application 層へ再配線
// pub mod entities;
mod infra;
pub mod routes;
pub mod util;

const MAJOR_VERSION: u32 = pkg_version_major!();
const MINOR_VERSION: u32 = pkg_version_minor!();
const PATCH_VERSION: u32 = pkg_version_patch!();
#[tokio::main]
async fn main() {
    // OpenAPI スキーマのダンプ(クライアント型生成用)。config/DB を要さず stdout へ JSON を出す。
    //   cargo run -- --dump-openapi=api_v3 > docs/api/api_v3/openapi.json
    //   cargo run -- --dump-openapi=auth_v2 > docs/api/auth_v2/openapi.json
    let args: Vec<String> = std::env::args().collect();
    if let Some(which) = args.iter().find_map(|a| a.strip_prefix("--dump-openapi=")) {
        let (title, mut openapi) = match which {
            "api_v3" => ("Koudaisai Portal API v3", routes::api_v3::router().split_for_parts().1),
            "auth_v2" => ("Koudaisai Portal Auth v2", routes::auth_v2::router().split_for_parts().1),
            other => {
                eprintln!("unknown schema '{other}' (expected api_v3 | auth_v2)");
                std::process::exit(2);
            }
        };
        openapi.info = utoipa::openapi::Info::new(title, env!("CARGO_PKG_VERSION"));
        println!(
            "{}",
            openapi.to_pretty_json().expect("serialize openapi as json")
        );
        return;
    }

    //初期化
    let config = init_config().unwrap();
    init_logging(config.logging);
    info!(
        "Koudaisai Portal v{}.{}.{} (c) 2025 JIZI All Rights Reserved.",
        MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION
    );

    // 各アダプタ/クライアントの初期化は infra 側の `from_config` コンストラクタが担う。
    // main は config を読み、構築結果を合成(composition root)するだけ。
    let oidc_client = crate::util::oidc::from_config(
        &config.web.auth.keycloak,
        format!("{}{}", &config.web.server.base_url, "/login"),
    )
    .await;

    // ===== v3 認証(/auth/v2)の composition root =====
    let pool = connect_and_migrate(&config.db.address).await.unwrap();
    let v3 = config.web.auth_v3.clone();

    let password_hasher = Argon2PasswordHasher::from_config(&config.web.auth_v3);
    let secret_generator = RandomSecretGenerator::from_config(&config.secrets);
    let access_issuer =
        JwtAccessTokenIssuer::from_config(&config.web.auth, &config.web.auth_v3).unwrap();

    // 定数時間ログイン用ダミー PHC は本番 argon2 で乱数をハッシュして生成する(監査 Low 対応)。
    let dummy_phc = {
        let plain = PlaintextPassword::new(secret_generator.generate_secret())
            .expect("generated secret satisfies length policy");
        password_hasher.hash(&plain).await.expect("hash dummy phc")
    };

    let prod_app = new_sqlite_application(
        pool.clone(),
        SendgridEmail::from_config(&config.sendgrid),
        S3ObjectStorage::from_config(&config.s3).await,
        WebhookDiscord::from_config(&config.discord),
        config.web.server.base_url.clone(),
        password_hasher,
        secret_generator,
        access_issuer,
    );

    let auth_v2_state = AuthV2State {
        app: Arc::new(prod_app),
        pool: pool.clone(),
        auth_config: AuthConfig {
            access_ttl: Duration::seconds(v3.access_token_ttl_secs),
            session_absolute_ttl: Duration::seconds(v3.session_absolute_ttl_secs),
            session_idle_ttl: Duration::seconds(v3.session_idle_ttl_secs),
            max_sessions_per_user: v3.max_sessions_per_user,
            activation_ttl: Duration::seconds(v3.activation_token_ttl_secs),
            reset_ttl: Duration::seconds(v3.reset_token_ttl_secs),
        },
        dummy_phc: Arc::new(dummy_phc),
        cookie: CookieConfig {
            name: v3.refresh_cookie_name.clone(),
            secure: v3.refresh_cookie_secure,
            same_site: v3.refresh_cookie_same_site.clone(),
        },
        email: Arc::new(SendgridEmail::from_config(&config.sendgrid)),
        reset_link_base: v3.reset_link_base.clone(),
        access_decoding_key: Arc::new(config.web.auth.get_jwt_decoding_key().unwrap()),
        access_iss: v3.access_token_iss.clone(),
        allowed_origins: Arc::new(v3.cors_allowed_origins.clone()),
        oidc_client: Arc::new(oidc_client),
        auth_sessions: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
        http_client: reqwest::Client::new(),
    };

    // credentials 付き CORS(`*` は使えないため origin は許可リスト)。
    let cors_origins: Vec<http::HeaderValue> = v3
        .cors_allowed_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();
    let auth_cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::list(cors_origins))
        .allow_credentials(true)
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::PATCH,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]);

    // auth_v2(事前認証) と api_v3(リソース系)は同じ `AuthV2State` を共有する
    // (`V3State = AuthV2State`)。State は 1 度だけ注入する。
    // legacy ルータ(静的配信 + /api/plans_info プロキシ)は撤去済み。
    let (auth_v2_router, _auth_openapi) = routes::auth_v2::router().split_for_parts();
    let (api_v3_router, _api_v3_openapi) = routes::api_v3::router().split_for_parts();
    let app = axum::Router::new()
        .nest("/auth/v2", auth_v2_router)
        .nest("/api/v3", api_v3_router)
        .with_state(auth_v2_state)
        .layer(auth_cors);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        &config.web.server.host, &config.web.server.port
    ))
    .await
    .unwrap();
    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

pub fn init_logging(logging: Logging) {
    if logging.json {
        tracing_subscriber::registry()
            .with(logging.log_level.to_level_filter())
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(logging.log_level.to_level_filter())
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}
