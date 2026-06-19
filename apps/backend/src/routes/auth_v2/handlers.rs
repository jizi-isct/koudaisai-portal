//! `/auth/v2/*` ハンドラ(事前認証エンドポイント)。
//!
//! リフレッシュトークンは `__Host-` Cookie(HttpOnly/Secure/SameSite=Strict/Path=/)で
//! やり取りし，アクセストークンは本文で返す(フロントはメモリ保持 + Authorization ヘッダ)。
//! 認証が必要な logout-all / password change はミドルウェア導入時に追加する。
//!
//! Cookie/カスタムヘッダを返すため戻り値は [`Response`]。OpenAPI は `#[utoipa::path]` の
//! `responses(...)` で記述する。

use super::dto::{
    ActivateRequest, ChangePasswordRequest, LoginRequest, PasswordResetConfirmRequest,
    PasswordResetRequest, TokenResponse,
};
use super::extract::{bearer, verify_access};
use super::{AuthV2State, CookieConfig};
use crate::application::auth::{AuthError, IssuedTokens};
use crate::application::ports::email::Email;
use crate::domain::email_address::EmailAddress;
use crate::domain::user::UserStatus;
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use tracing::warn;

fn auth_status(e: &AuthError) -> StatusCode {
    match e {
        AuthError::InvalidCredentials
        | AuthError::InvalidToken
        | AuthError::ReuseDetected
        | AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
        AuthError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        AuthError::Conflict => StatusCode::CONFLICT,
        AuthError::Internal(err) => {
            warn!("auth internal error: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

fn set_cookie_value(cfg: &CookieConfig, value: &str, max_age_secs: i64) -> String {
    let secure = if cfg.secure { "; Secure" } else { "" };
    format!(
        "{}={}; HttpOnly{secure}; Path=/; SameSite={}; Max-Age={}",
        cfg.name, value, cfg.same_site, max_age_secs
    )
}

fn cleared_cookie(cfg: &CookieConfig) -> String {
    let secure = if cfg.secure { "; Secure" } else { "" };
    format!(
        "{}=; HttpOnly{secure}; Path=/; SameSite={}; Max-Age=0",
        cfg.name, cfg.same_site
    )
}

fn read_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    raw.split(';').find_map(|part| {
        let part = part.trim();
        part.strip_prefix(name)
            .and_then(|rest| rest.strip_prefix('='))
            .map(|v| v.to_string())
    })
}

/// CSRF 対策(SameSite=Strict の多重防御)。Origin が来た場合は許可リストと厳密一致を要求。
/// Origin が無い(同一オリジン/非ブラウザ)場合は許可する。
fn origin_allowed(headers: &HeaderMap, allowed: &[String]) -> bool {
    match headers.get(header::ORIGIN).and_then(|v| v.to_str().ok()) {
        Some(origin) => allowed.iter().any(|a| a == origin),
        None => true,
    }
}

fn with_set_cookie(mut resp: Response, cookie: String) -> Response {
    if let Ok(value) = header::HeaderValue::from_str(&cookie) {
        resp.headers_mut().append(header::SET_COOKIE, value);
    }
    resp
}

fn issued_response(st: &AuthV2State, tokens: IssuedTokens) -> Response {
    let max_age = (tokens.refresh_expires_at - Utc::now())
        .num_seconds()
        .max(0);
    let cookie = set_cookie_value(&st.cookie, &tokens.refresh_token, max_age);
    let body = Json(TokenResponse {
        access_token: tokens.access_token,
        token_type: "Bearer".to_string(),
        expires_in: tokens.access_ttl.num_seconds(),
    });
    with_set_cookie((StatusCode::OK, body).into_response(), cookie)
}

/// アクセストークンを検証し，状態ゲート(Active かつ iat>=password_changed_at)を
/// 通ったユーザ ID を返す。失敗は 401。
async fn authenticate(headers: &HeaderMap, st: &AuthV2State) -> Result<UserId, StatusCode> {
    let token = bearer(headers).ok_or(StatusCode::UNAUTHORIZED)?;
    let claims = verify_access(&token, &st.access_decoding_key, &st.access_iss)?;
    let user_id = UserId::new(claims.sub);
    let user = st
        .app
        .find_user_for_auth(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    // パスワード変更後に発行前トークンが残るのを弾く + 無効化済みユーザを弾く。
    match user.status() {
        UserStatus::Active {
            password_credentials,
        } if claims.iat >= password_credentials.changed_at().timestamp() => Ok(user_id),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

#[utoipa::path(
    post,
    path = "/login",
    tag = super::AUTH_TAG,
    request_body = LoginRequest,
    responses(
        (status = OK, description = "ログイン成功。リフレッシュトークンは Set-Cookie で配送", body = TokenResponse),
        (status = UNAUTHORIZED, description = "認証失敗(存在/状態を区別しない汎用)"),
    ),
)]
pub async fn login(State(st): State<AuthV2State>, Json(body): Json<LoginRequest>) -> Response {
    // メール形式不正も列挙を避けて汎用 401。
    let Ok(m_address) = EmailAddress::new(body.m_address) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let tx = SqliteTransaction::new(st.pool.clone());
    let auth = st.app.auth(st.auth_config.clone(), (*st.dummy_phc).clone());
    match auth.login(tx, &m_address, &body.password).await {
        Ok(tokens) => issued_response(&st, tokens),
        Err(e) => auth_status(&e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/refresh",
    tag = super::AUTH_TAG,
    responses(
        (status = OK, description = "回転成功。新リフレッシュトークンを Set-Cookie", body = TokenResponse),
        (status = UNAUTHORIZED, description = "トークン無効 / reuse 検知 / Cookie 無し"),
    ),
)]
pub async fn refresh(State(st): State<AuthV2State>, headers: HeaderMap) -> Response {
    if !origin_allowed(&headers, &st.allowed_origins) {
        return StatusCode::FORBIDDEN.into_response();
    }
    let Some(refresh_token) = read_cookie(&headers, &st.cookie.name) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let tx = SqliteTransaction::new(st.pool.clone());
    let auth = st.app.auth(st.auth_config.clone(), (*st.dummy_phc).clone());
    match auth.refresh(tx, &refresh_token).await {
        Ok(tokens) => issued_response(&st, tokens),
        Err(AuthError::ReuseDetected) => {
            // 盗難検知: ファミリは失効済み。Cookie をクリアして再ログインを促す。
            // TODO(discord): Application の discord ポート経由で運用者へ通知する。
            warn!("refresh token reuse detected; session family revoked");
            with_set_cookie(
                StatusCode::UNAUTHORIZED.into_response(),
                cleared_cookie(&st.cookie),
            )
        }
        Err(e) => with_set_cookie(auth_status(&e).into_response(), cleared_cookie(&st.cookie)),
    }
}

#[utoipa::path(
    post,
    path = "/logout",
    tag = super::AUTH_TAG,
    responses((status = NO_CONTENT, description = "ログアウト(冪等)。Cookie をクリア")),
)]
pub async fn logout(State(st): State<AuthV2State>, headers: HeaderMap) -> Response {
    if !origin_allowed(&headers, &st.allowed_origins) {
        return StatusCode::FORBIDDEN.into_response();
    }
    if let Some(refresh_token) = read_cookie(&headers, &st.cookie.name) {
        let tx = SqliteTransaction::new(st.pool.clone());
        let auth = st.app.auth(st.auth_config.clone(), (*st.dummy_phc).clone());
        let _ = auth.logout(tx, &refresh_token).await; // 冪等・best-effort
    }
    with_set_cookie(
        StatusCode::NO_CONTENT.into_response(),
        cleared_cookie(&st.cookie),
    )
}

#[utoipa::path(
    post,
    path = "/activate",
    tag = super::AUTH_TAG,
    request_body = ActivateRequest,
    responses(
        (status = NO_CONTENT, description = "有効化成功(初回パスワード設定)"),
        (status = UNAUTHORIZED, description = "トークン無効/失効/消費済み"),
        (status = BAD_REQUEST, description = "パスワードポリシー違反"),
        (status = CONFLICT, description = "既に有効化済み"),
    ),
)]
pub async fn activate(
    State(st): State<AuthV2State>,
    Json(body): Json<ActivateRequest>,
) -> Response {
    let tx = SqliteTransaction::new(st.pool.clone());
    let auth = st.app.auth(st.auth_config.clone(), (*st.dummy_phc).clone());
    match auth.activate(tx, &body.token, &body.password).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => auth_status(&e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/password/reset",
    tag = super::AUTH_TAG,
    request_body = PasswordResetRequest,
    responses((status = ACCEPTED, description = "常に 202(存在/不在を漏らさない)")),
)]
pub async fn password_reset(
    State(st): State<AuthV2State>,
    Json(body): Json<PasswordResetRequest>,
) -> Response {
    if let Ok(m_address) = EmailAddress::new(body.m_address) {
        let tx = SqliteTransaction::new(st.pool.clone());
        let auth = st.app.auth(st.auth_config.clone(), (*st.dummy_phc).clone());
        if let Ok(Some(issued)) = auth.request_password_reset(tx, &m_address).await {
            // メール送信はリクエストパス外(spawn)で best-effort。応答時間で存在を漏らさない。
            let email = st.email.clone();
            let base = st.reset_link_base.clone();
            tokio::spawn(async move {
                let link = format!("{base}?token={}", issued.raw_token);
                let subject = "【工大祭ポータル】パスワード再設定";
                let body = format!(
                    "以下のリンクからパスワードを再設定してください(30分間有効)。\n{link}\n\n\
                     心当たりがない場合はこのメールを破棄してください。\n"
                );
                if let Err(e) = email.send(&issued.m_address, subject, &body).await {
                    warn!("failed to send password reset email: {e:?}");
                }
            });
        }
    }
    StatusCode::ACCEPTED.into_response()
}

#[utoipa::path(
    post,
    path = "/password/reset/confirm",
    tag = super::AUTH_TAG,
    request_body = PasswordResetConfirmRequest,
    responses(
        (status = NO_CONTENT, description = "リセット成功。全セッション失効・Cookie クリア"),
        (status = UNAUTHORIZED, description = "トークン無効/失効/消費済み"),
        (status = BAD_REQUEST, description = "パスワードポリシー違反"),
    ),
)]
pub async fn password_reset_confirm(
    State(st): State<AuthV2State>,
    Json(body): Json<PasswordResetConfirmRequest>,
) -> Response {
    let tx = SqliteTransaction::new(st.pool.clone());
    let auth = st.app.auth(st.auth_config.clone(), (*st.dummy_phc).clone());
    match auth
        .confirm_password_reset(tx, &body.token, &body.new_password)
        .await
    {
        // 全セッション失効済みなので Cookie もクリアして再ログインさせる。
        Ok(()) => with_set_cookie(
            StatusCode::NO_CONTENT.into_response(),
            cleared_cookie(&st.cookie),
        ),
        Err(e) => auth_status(&e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/logout-all",
    tag = super::AUTH_TAG,
    responses(
        (status = NO_CONTENT, description = "全デバイスのセッションを失効。Cookie クリア"),
        (status = UNAUTHORIZED, description = "アクセストークン無効"),
    ),
)]
pub async fn logout_all(State(st): State<AuthV2State>, headers: HeaderMap) -> Response {
    let user_id = match authenticate(&headers, &st).await {
        Ok(u) => u,
        Err(s) => return s.into_response(),
    };
    let tx = SqliteTransaction::new(st.pool.clone());
    let auth = st.app.auth(st.auth_config.clone(), (*st.dummy_phc).clone());
    match auth.logout_all(tx, user_id).await {
        Ok(()) => with_set_cookie(
            StatusCode::NO_CONTENT.into_response(),
            cleared_cookie(&st.cookie),
        ),
        Err(e) => auth_status(&e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/password/change",
    tag = super::AUTH_TAG,
    request_body = ChangePasswordRequest,
    responses(
        (status = NO_CONTENT, description = "変更成功。全セッション失効・Cookie クリア"),
        (status = UNAUTHORIZED, description = "アクセストークン無効 / 旧パスワード不一致"),
        (status = BAD_REQUEST, description = "新パスワードがポリシー違反"),
    ),
)]
pub async fn password_change(
    State(st): State<AuthV2State>,
    headers: HeaderMap,
    Json(body): Json<ChangePasswordRequest>,
) -> Response {
    let user_id = match authenticate(&headers, &st).await {
        Ok(u) => u,
        Err(s) => return s.into_response(),
    };
    let tx = SqliteTransaction::new(st.pool.clone());
    let auth = st.app.auth(st.auth_config.clone(), (*st.dummy_phc).clone());
    match auth
        .change_password(tx, user_id, &body.old_password, &body.new_password)
        .await
    {
        Ok(()) => with_set_cookie(
            StatusCode::NO_CONTENT.into_response(),
            cleared_cookie(&st.cookie),
        ),
        Err(e) => auth_status(&e).into_response(),
    }
}
