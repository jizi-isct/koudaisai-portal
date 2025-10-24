use crate::entities::user::UserRead;
use crate::routes::AppState;
use crate::sea_orm_entities::users;
use crate::util::AppResponse;
use crate::util::sha::digest;
use anyhow::anyhow;
use axum::Json;
use axum::Router;
use axum::extract::{ConnectInfo, State};
use axum::response::IntoResponse;
use axum::routing::post;
use axum_gcra::RateLimitLayer;
use axum_gcra::gcra::Quota;
use axum_gcra::real_ip::RealIp;
use http::StatusCode;
use jsonwebtoken::errors::ErrorKind;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::instrument;
use tracing::{debug, warn};

#[instrument(name = "init /auth/v1/password")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/change",
            post(post_change).route_layer(
                RateLimitLayer::<RealIp>::builder()
                    .with_default_quota(Quota::simple(Duration::from_secs(10)))
                    .with_global_fallback(true)
                    .with_gc_interval(Duration::from_secs(5))
                    .default_handle_error(),
            ),
        )
        .route(
            "/reset",
            post(post_reset).route_layer(
                RateLimitLayer::<RealIp>::builder()
                    .with_default_quota(Quota::simple(Duration::from_secs(10)))
                    .with_global_fallback(true)
                    .with_gc_interval(Duration::from_secs(5))
                    .default_handle_error(),
            ),
        )
        .route(
            "/reset/confirm",
            post(post_reset_confirm).route_layer(
                RateLimitLayer::<RealIp>::builder()
                    .with_default_quota(Quota::simple(Duration::from_secs(10)))
                    .with_global_fallback(true)
                    .with_gc_interval(Duration::from_secs(5))
                    .default_handle_error(),
            ),
        )
}

#[derive(Serialize, Deserialize)]
struct PasswordChangePayload {
    access_token: String,
    old_password: String,
    new_password: String,
}

#[instrument(name = "POST /auth/v1/password/change", skip(state, payload))]
#[axum::debug_handler]
async fn post_change(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PasswordChangePayload>,
) -> StatusCode {
    let access_token = match state.jwt_manager.decode(payload.access_token.as_str()) {
        Ok(access_token) => access_token,
        Err(_) => {
            return StatusCode::UNAUTHORIZED;
        }
    };

    let user = match UserRead::find_from_id(access_token.claims.sub, &state.db_conn).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!("jwt token with invalid user was provided.");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        Err(err) => {
            warn!("internal error occurred while executing sql: {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if state
        .jwt_manager
        .is_access_token_valid(&access_token.claims, &user)
    {
        let sub = access_token.claims.sub;
        let user = match users::Entity::find_by_id(sub).one(&state.db_conn).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                warn!("jwt token with invalid user was provided.");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            Err(err) => {
                warn!("internal error occurred while executing sql: {:?}", err);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };
        //有効化されていない
        if user.password_hash == None {
            debug!("The account wasn't activated");
            return StatusCode::UNAUTHORIZED;
        }
        let current_pwd_hash = user.password_hash.clone().unwrap();
        let old_pwd_hash = state.sha_manager.stretch_with_salt(
            payload.old_password.to_string().as_str(),
            user.password_salt.as_str(),
        );
        let new_pwd_hash = state.sha_manager.stretch_with_salt(
            payload.new_password.to_string().as_str(),
            user.password_salt.as_str(),
        );

        if digest(old_pwd_hash.as_str()) == digest(current_pwd_hash.as_str()) {
            let mut user = user.into_active_model();
            user.password_hash = Set(Some(new_pwd_hash));
            match user.update(&state.db_conn).await {
                Ok(_) => StatusCode::OK,
                Err(err) => {
                    warn!(
                        "Internal server error occurred while updating user password: {:?}",
                        err
                    );
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        } else {
            debug!("password incorrect");
            StatusCode::UNAUTHORIZED
        }
    } else {
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Serialize, Deserialize)]
struct PasswordResetPayload {
    m_address: String,
}

#[instrument(
    name = "POST /auth/v1/password/reset",
    fields(payload.m_address = %payload.m_address),
    skip(payload, state)
)]
#[axum::debug_handler]
async fn post_reset(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PasswordResetPayload>,
) -> AppResponse {
    // Check if user exists
    let user =
        match UserRead::find_from_m_address(payload.m_address.clone(), &state.db_conn).await? {
            Some(user) => user,
            None => {
                // If user does not exist, we still return OK to prevent information leakage
                debug!(
                    "Password reset requested for non-existent user: {}",
                    payload.m_address
                );
                return Ok((StatusCode::NO_CONTENT, "".into_response()));
            }
        };

    // Generate a password reset token
    let reset_token = state.jwt_manager.issue_password_reset_token(user.id)?;

    // メール送信
    let response = user
        .send_password_reset_email(
            &state.sendgrid_sender,
            state.sendgrid_sender_email.clone(),
            reset_token,
            state.jwt_manager.password_reset_token_expire_time.clone(),
        )
        .await;

    // エラーハンドリング
    match response {
        Ok(response) => match response.error_for_status() {
            Ok(_) => debug!("Password reset email sent successfully"),
            Err(err) => {
                warn!("Failed to send password reset email: {:?}", err);
            }
        },
        Err(err) => {
            warn!("Failed to send password reset email: {:?}", err);
        }
    }

    Ok((StatusCode::NO_CONTENT, "".into_response()))
}

#[derive(Serialize, Deserialize)]
struct PasswordResetConfirmPayload {
    reset_token: String,
    new_password: String,
}

#[instrument(name = "POST /auth/v1/password/reset/confirm", skip(state, payload))]
#[axum::debug_handler]
async fn post_reset_confirm(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PasswordResetConfirmPayload>,
) -> AppResponse {
    let claims = match state
        .jwt_manager
        .decode_password_reset_token(payload.reset_token.as_str())
    {
        Ok(claims) => claims,
        Err(err) => match err.into_kind() {
            ErrorKind::InvalidToken => {
                warn!("Invalid password reset token: {}", payload.reset_token);
                return Ok((StatusCode::UNAUTHORIZED, "".into_response()));
            }
            ErrorKind::InvalidSignature => {
                warn!(
                    "Invalid signature for password reset token: {}",
                    payload.reset_token
                );
                return Ok((StatusCode::UNAUTHORIZED, "".into_response()));
            }
            ErrorKind::InvalidKeyFormat => {
                warn!(
                    "Invalid key format for password reset token: {}",
                    payload.reset_token
                );
                return Ok((StatusCode::UNAUTHORIZED, "".into_response()));
            }
            ErrorKind::ExpiredSignature => {
                warn!("Password reset token expired: {}", payload.reset_token);
                return Ok((StatusCode::UNAUTHORIZED, "".into_response()));
            }
            e => {
                return Err(anyhow!(
                    "internal error occurred while decoding password reset token: {:?}",
                    e
                )
                .into());
            }
        },
    }
    .claims;

    let user = match UserRead::find_from_id(claims.sub, &state.db_conn).await? {
        Some(user) => user,
        None => {
            warn!("Password reset token for non-existent user: {}", claims.sub);
            return Ok((StatusCode::NOT_FOUND, "".into_response()));
        }
    };

    debug!("Password reset confirmed for {}", claims.sub);

    user.change_password(&state.db_conn, payload.new_password, &state.sha_manager)
        .await?;

    Ok((StatusCode::NO_CONTENT, "".into_response()))
}
