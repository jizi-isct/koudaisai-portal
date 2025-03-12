use crate::entities::prelude::Users;
use crate::entities::users;
use crate::routes::AppState;
use crate::util::jwt;
use crate::util::jwt::Role;
use crate::util::sha::{digest, stretch_with_salt};
use axum::extract::{ConnectInfo, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::{cookie, CookieJar};
use axum_gcra::gcra::Quota;
use axum_gcra::real_ip::RealIp;
use axum_gcra::RateLimitLayer;
use chrono::Utc;
use http::HeaderValue;
use openid::{Client, Options, Token, Userinfo};
use rand::distr::{Alphanumeric, SampleString};
use rand::rng;
use reqwest::Url;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, instrument, warn};
use uuid::Uuid;

#[instrument(name = "init /auth")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/activate", post(activate))
        .route("/v1/login", get(login))
        .route_layer(
            RateLimitLayer::<RealIp>::builder()
                .with_default_quota(Quota::simple(Duration::from_secs(10)))
                .with_global_fallback(true)
                .with_gc_interval(Duration::from_secs(5))
                .default_handle_error(),
        )
}

#[derive(Serialize, Deserialize)]
struct ActivatePayload {
    uuid: Uuid,
    token: String,
    password: String,
}
#[instrument(name = "/auth/v1/activate", fields(payload.uuid = %payload.uuid), skip(payload, state))]
async fn activate(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ActivatePayload>,
) -> StatusCode {
    let right_token = stretch_with_salt(
        payload.uuid.to_string().as_str(),
        state.web.auth.activation_salt.as_str(),
        2_i32.pow(state.web.auth.stretch_cost as u32) as u32,
    )
    .await;

    if digest(&*payload.token) == digest(&*right_token) {
        //文字列比較の計算時間からトークンを推測されないようにdigestしてから比較
        let user = match Users::find_by_id(payload.uuid).one(&state.db_conn).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                debug!("404 Not Found");
                return StatusCode::NOT_FOUND;
            }
            Err(err) => {
                warn!("internal server error occurred while finding user: {}", err);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };

        //すでに有効化されているかどうかを確認
        if user.password_hash.is_some() {
            debug!("409 Conflict");
            return StatusCode::CONFLICT;
        }

        let password_salt = (&user.password_salt).to_string();

        let mut user: users::ActiveModel = user.into();

        user.password_hash = Set(Some(
            stretch_with_salt(
                payload.password.to_string().as_str(),
                &*password_salt,
                2_i32.pow(state.web.auth.stretch_cost as u32) as u32,
            )
            .await,
        ));
        match user.update(&state.db_conn).await {
            Ok(_) => StatusCode::OK,
            Err(err) => {
                warn!(
                    "internal server error occurred while updating user: {}",
                    err
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    } else {
        debug!("401 Unauthorized");
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Serialize, Deserialize)]
struct LoginPayload {
    m_address: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[instrument(name = "/auth/v1/login", fields(payload.m_address = %payload.m_address), skip(payload, state))]
async fn login(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<CookieJar, StatusCode> {
    let user = match Users::find()
        .filter(users::Column::MAddress.eq(payload.m_address))
        .one(&state.db_conn)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            debug!("401 Unauthorized(user)");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(err) => {
            warn!("internal server error occurred while finding user: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let prompted_hash = stretch_with_salt(
        payload.password.to_string().as_str(),
        state.web.auth.password_salt.as_str(),
        2_i32.pow(state.web.auth.stretch_cost as u32) as u32,
    )
    .await;

    let password_hash = match user.password_hash {
        Some(hash) => hash,
        None => {
            debug!("401 Unauthorized(not activated)");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if digest(&*prompted_hash) == digest(&*password_hash) {
        match jwt::issue_cookie(
            user.id.to_string(),
            Role::User,
            &state.jwt_encoding_key,
            state.web.server.host.clone(),
        ) {
            Ok(jar) => Ok(jar),
            Err(err) => {
                warn!("internal server error while generarting tokens: {}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        debug!("401 Unauthorized(password)");
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[instrument(name = "/auth/v1/admin/login", skip(state))]
async fn admin_login(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, HeaderMap) {
    //url
    let url = state.oidc_client.auth_url(&Options {
        ..Default::default()
    });
    let url = url.to_string();

    //header
    let mut headers = HeaderMap::new();
    let val = if let Ok(val) = HeaderValue::from_str(&url) {
        val
    } else {
        warn!("internal server error while setting header");
        return (StatusCode::INTERNAL_SERVER_ERROR, headers);
    };
    headers.insert(http::header::LOCATION, val);

    (StatusCode::FOUND, headers)
}

#[derive(Serialize, Deserialize)]
struct RedirectQuery {
    pub code: String,
    pub state: String,
}
#[instrument(name = "/auth/v1/admin/redirect", skip(state, login_query))]
async fn admin_redirect(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    login_query: Query<RedirectQuery>,
) -> Result<CookieJar, StatusCode> {
    let (token, _userinfo) = match request_token(&state.oidc_client, &login_query.code).await {
        Ok(Some(tuple)) => tuple,
        Ok(None) => {
            warn!("login error in call: no id_token found");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Err(err) => {
            warn!("login error in call: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut id_token = token.id_token.unwrap();
    let payload = id_token.payload_mut().unwrap();
    match jwt::issue_cookie(
        payload.sub.clone(),
        Role::Admin,
        &state.jwt_encoding_key,
        state.web.server.host.clone(),
    ) {
        Ok(jar) => Ok(jar),
        Err(err) => {
            warn!("internal server error while generarting tokens: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn request_token(
    oidc_client: &Client,
    code: &String,
) -> Result<Option<(Token, Userinfo)>, ()> {
    let mut token: Token = oidc_client
        .request_token(&code)
        .await
        .map_err(|_| ())?
        .into();

    if let Some(id_token) = token.id_token.as_mut() {
        oidc_client.decode_token(id_token).map_err(|_| ())?;
        oidc_client
            .validate_token(&id_token, None, None)
            .map_err(|_| ())?;
    } else {
        return Ok(None);
    }

    let userinfo = oidc_client.request_userinfo(&token).await.map_err(|_| ())?;

    Ok(Some((token, userinfo)))
}
