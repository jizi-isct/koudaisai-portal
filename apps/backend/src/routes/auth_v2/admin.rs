//! 管理者(JIZI)向け Keycloak OIDC ログイン。legacy `/auth/v1/admin/*` を挙動不変で移管。
//!
//! Group 認証(argon2 + 回転セッション)とは別系統で，Keycloak が発行したトークンを
//! そのまま返す。回転セッション/Cookie の仕組みは使わない。

use super::AuthV2State;
use crate::util::oidc::OIDCClient;
use anyhow::Result;
use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use oauth2::{
    AccessToken, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RefreshToken,
    Scope, TokenResponse,
};
use openidconnect::Nonce;
use openidconnect::core::CoreAuthenticationFlow;
// OIDC 系は openidconnect/oauth2 が依存する reqwest(0.12) の Client を使う。
// クレート直下の `reqwest` は 0.13 で、生成クライアント(events26_api)と共用しているため別物。
use openidconnect::reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use tracing::warn;
use utoipa::ToSchema;

/// リダイレクト確定までの間，CSRF state をキーに保持する PKCE verifier と nonce。
pub struct AdminAuthSession {
    pub pkce_verifier: PkceCodeVerifier,
    pub nonce: Nonce,
}

#[derive(Deserialize, ToSchema)]
pub struct AdminRedirectRequest {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, ToSchema)]
pub struct AdminTokenResponse {
    pub refresh_token: String,
    pub access_token: String,
}

#[utoipa::path(
    get,
    path = "/admin/login",
    tag = super::AUTH_TAG,
    responses((status = FOUND, description = "Keycloak 認可エンドポイントへリダイレクト")),
)]
pub async fn admin_login(State(st): State<AuthV2State>) -> Response {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (url, csrf_token, nonce) = st
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
    st.auth_sessions.lock().await.insert(
        csrf_token.secret().clone(),
        AdminAuthSession {
            pkce_verifier,
            nonce,
        },
    );

    let Ok(location) = header::HeaderValue::from_str(url.as_str()) else {
        warn!("failed to build admin login redirect header");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, location);
    (StatusCode::FOUND, headers).into_response()
}

#[utoipa::path(
    post,
    path = "/admin/redirect",
    tag = super::AUTH_TAG,
    request_body = AdminRedirectRequest,
    responses(
        (status = OK, description = "Keycloak トークン", body = AdminTokenResponse),
        (status = BAD_REQUEST, description = "未知の state"),
    ),
)]
pub async fn admin_redirect(
    State(st): State<AuthV2State>,
    Json(payload): Json<AdminRedirectRequest>,
) -> Response {
    let Some(auth_session) = st.auth_sessions.lock().await.remove(&payload.state) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    match request_token(
        auth_session,
        &st.http_client,
        &st.oidc_client,
        &payload.code,
    )
    .await
    {
        Ok((refresh_token, access_token)) => Json(AdminTokenResponse {
            refresh_token: refresh_token.into_secret(),
            access_token: access_token.into_secret(),
        })
        .into_response(),
        Err(err) => {
            warn!("admin oidc token exchange failed: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
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
        write!(f, "{self:?}")
    }
}

impl Error for RequestTokenError {}

async fn request_token(
    auth_session: AdminAuthSession,
    http_client: &Client,
    oidc_client: &OIDCClient,
    code: &str,
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

    // id_token の nonce 検証。
    let id_token = token_response
        .extra_fields()
        .id_token()
        .ok_or(RequestTokenError::NoIdTokenError)?;
    id_token.claims(&oidc_client.id_token_verifier(), &auth_session.nonce)?;

    Ok((refresh_token, access_token))
}
