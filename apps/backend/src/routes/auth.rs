mod password;

use crate::routes::{AppState, AuthSession};
use crate::sea_orm_entities::prelude::Users;
use crate::sea_orm_entities::{revoked_refresh_tokens, users};
use crate::util::jwt;
use crate::util::oidc::OIDCClient;
use crate::util::sha::{digest, stretch_with_salt};
use anyhow::Result;
use axum::extract::{ConnectInfo, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_gcra::gcra::Quota;
use axum_gcra::real_ip::RealIp;
use axum_gcra::RateLimitLayer;
use http::HeaderValue;
use oauth2::{
    AccessToken, AuthorizationCode, CsrfToken, PkceCodeChallenge, RefreshToken, Scope,
    TokenResponse,
};
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::Nonce;
use reqwest::Client;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{ColumnTrait};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, instrument, warn};

#[instrument(name = "init /auth")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/v1/activate",
            post(activate).route_layer(
                RateLimitLayer::<RealIp>::builder()
                    .with_default_quota(Quota::simple(Duration::from_secs(10)))
                    .with_global_fallback(true)
                    .with_gc_interval(Duration::from_secs(5))
                    .default_handle_error(),
            ),
        )
        .route(
            "/v1/login",
            post(login).route_layer(
                RateLimitLayer::<RealIp>::builder()
                    .with_default_quota(Quota::simple(Duration::from_secs(10)))
                    .with_global_fallback(true)
                    .with_gc_interval(Duration::from_secs(5))
                    .default_handle_error(),
            ),
        )
        .route("/v1/refresh", post(refresh))
        .route("/v1/revoke", post(revoke))
        .route("/v1/admin/login", get(admin_login))
        .route("/v1/admin/redirect", post(admin_redirect))
        .nest("/v1/password", password::init_router())
}

#[derive(Serialize, Deserialize)]
struct ActivatePayload {
    m_address: String,
    token: String,
    password: String,
}
#[instrument(name = "/auth/v1/activate", fields(payload.m_address = %payload.m_address), skip(
    payload,
    state
))]
async fn activate(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ActivatePayload>,
) -> StatusCode {
    let right_token = stretch_with_salt(
        payload.m_address.to_string().as_str(),
        state.web.auth.activation_salt.as_str(),
        2_i32.pow(state.web.auth.stretch_cost as u32),
    );

    if digest(&*payload.token) == digest(&*right_token) {
        //文字列比較の計算時間からトークンを推測されないようにdigestしてから比較
        let user = match Users::find()
            .filter(users::Column::MAddress.eq(payload.m_address.to_string()))
            .one(&state.db_conn)
            .await
        {
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

        user.password_hash = Set(Some(stretch_with_salt(
            payload.password.to_string().as_str(),
            &*password_salt,
            2_i32.pow(state.web.auth.stretch_cost as u32),
        )));
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

#[instrument(name = "/auth/v1/login", fields(payload.m_address = %payload.m_address), skip(payload, state))]
async fn login(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<jwt::Tokens>, StatusCode> {
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

    let prompted_hash = state.sha_manager.stretch_with_salt(
        payload.password.to_string().as_str(),
        user.password_salt.as_str(),
    );

    let password_hash = match user.password_hash {
        Some(hash) => hash,
        None => {
            debug!("401 Unauthorized(not activated)");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if digest(&*prompted_hash) == digest(&*password_hash) {
        match state.jwt_manager.issue_tokens(user.id) {
            Ok(tokens) => Ok(Json(tokens)),
            Err(err) => {
                warn!("internal server error while generating tokens: {}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        debug!("401 Unauthorized(password)");
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Serialize, Deserialize)]
struct RefreshPayload {
    refresh_token: String,
}
#[derive(Serialize, Deserialize)]
struct RefreshResponse {
    access_token: String,
}

#[instrument(name = "/auth/v1/refresh", skip(state, payload))]
async fn refresh(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshPayload>,
) -> Result<Json<RefreshResponse>, StatusCode> {
    let refresh_token = match state.jwt_manager.decode(payload.refresh_token.as_str()) {
        Ok(token) => token,
        Err(err) => {
            debug!("token decoding failed: {:?}", err);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    let claims = refresh_token.claims;

    if match state
        .jwt_manager
        .is_refresh_token_valid(payload.refresh_token, &claims)
        .await
    {
        Ok(is_valid) => is_valid,
        Err(err) => {
            warn!("Token verification failed: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    } {
        match state.jwt_manager.issue_access_token(claims.sub) {
            Ok(access_token) => Ok(Json(RefreshResponse { access_token })),
            Err(err) => {
                warn!("internal server error while generating tokens: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Serialize, Deserialize)]
struct RevokePayload {
    refresh_token: String,
}
#[instrument(name = "/auth/v1/revoke", skip(state, payload))]
async fn revoke(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RevokePayload>,
) -> StatusCode {
    // refresh_tokenの有効性確認
    let refresh_token = match state.jwt_manager.decode(payload.refresh_token.as_str()) {
        Ok(token) => token,
        Err(_) => {
            return StatusCode::UNAUTHORIZED;
        }
    };
    let is_valid = match state
        .jwt_manager
        .is_refresh_token_valid(payload.refresh_token.clone(), &refresh_token.claims)
        .await
    {
        Ok(is_valid) => is_valid,
        Err(err) => {
            warn!("Internal server error occurred: {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    if !is_valid {
        return StatusCode::UNAUTHORIZED;
    }

    // refresh_tokenの失効
    let model = revoked_refresh_tokens::ActiveModel {
        refresh_token: Set(payload.refresh_token),
        exp: Set(refresh_token.claims.exp as i32),
    };
    match revoked_refresh_tokens::Entity::insert(model)
        .exec(&state.db_conn)
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            warn!("Internal server error occurred: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[instrument(name = "/auth/v1/admin/login", skip(state))]
async fn admin_login(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, HeaderMap) {
    //url発行
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (url, csrf_token, nonce) = state
        .oidc_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    state.auth_sessions.lock().await.insert(
        csrf_token.secret().clone(),
        AuthSession {
            pkce_verifier,
            nonce,
        },
    );

    //header
    let mut headers = HeaderMap::new();
    let val = if let Ok(val) = HeaderValue::from_str(url.as_str()) {
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

#[derive(Serialize, Deserialize)]
struct RedirectResponse {
    pub refresh_token: String,
    pub access_token: String,
}

#[instrument(name = "/auth/v1/admin/redirect", skip(state, payload))]
async fn admin_redirect(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    payload: Json<RedirectQuery>,
) -> Result<Json<RedirectResponse>, StatusCode> {
    let auth_session = match state.auth_sessions.lock().await.remove(&payload.state) {
        Some(auth_session) => auth_session,
        None => Err(StatusCode::BAD_REQUEST)?,
    };
    let (refresh_token, access_token) = match request_token(
        auth_session,
        &state.http_client,
        &state.oidc_client,
        &payload.code,
    )
    .await
    {
        Ok(tuple) => tuple,
        Err(err) => {
            warn!("login error in call: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json::from(RedirectResponse {
        refresh_token: refresh_token.into_secret(),
        access_token: access_token.into_secret(),
    }))
}

enum RequestTokenError {
    NoRefreshTokenError,
    NoIdTokenError,
}

impl Debug for RequestTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoRefreshTokenError => f.write_str("NoRefreshTokenError"),
            Self::NoIdTokenError => f.write_str("NoIdTokenError"),
        }
    }
}

impl Display for RequestTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RequestTokenError {}

async fn request_token(
    auth_session: AuthSession,
    http_client: &Client,
    oidc_client: &OIDCClient,
    code: &String,
) -> Result<(RefreshToken, AccessToken)> {
    let token_response = oidc_client
        .exchange_code(AuthorizationCode::new(code.to_string()))?
        .set_pkce_verifier(auth_session.pkce_verifier)
        .request_async(http_client)
        .await?;

    let refresh_token = token_response
        .refresh_token()
        .ok_or(RequestTokenError::NoRefreshTokenError)?
        .clone();
    let access_token = token_response.access_token().clone();

    // id_tokenのnonce検証
    let id_token = token_response
        .extra_fields()
        .id_token()
        .ok_or(RequestTokenError::NoIdTokenError)?;
    id_token.claims(&oidc_client.id_token_verifier(), &auth_session.nonce)?;

    Ok((refresh_token, access_token))
}
