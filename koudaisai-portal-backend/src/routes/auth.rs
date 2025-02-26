use crate::entities::prelude::Users;
use crate::entities::users;
use crate::routes::AppState;
use crate::util::jwt;
use crate::util::sha::{digest, stretch_with_salt};
use axum::extract::{ConnectInfo, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use axum_gcra::gcra::Quota;
use axum_gcra::real_ip::RealIp;
use axum_gcra::RateLimitLayer;
use chrono::Utc;
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

#[instrument(name = "init /auth", skip(state))]
pub fn init_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/activate", post(activate))
        .route_layer(
            RateLimitLayer::<RealIp>::builder()
                .with_default_quota(Quota::simple(Duration::from_secs(10)))
                .with_global_fallback(true)
                .with_gc_interval(Duration::from_secs(5))
                .default_handle_error(),
        )
        .with_state(Arc::clone(&state))
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

struct LoginPayload {
    m_address: String,
    password: String,
}

struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[instrument(name = "/auth/v1/login", fields(payload.m_address = %payload.m_address), skip(payload, state))]
async fn login(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, StatusCode> {
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
        let access_token_claim = jwt::Claims {
            exp: Utc::now().timestamp() as usize + jwt::ACCESS_TOKEN_EXPIRE_TIME,
            iss: "jizi".parse().unwrap(),
            typ: jwt::AccountType::User,
        };
        let access_token = match jwt::encode(&access_token_claim, &state.jwt_encoding_key) {
            Ok(token) => token,
            Err(err) => {
                warn!(
                    "internal server error occurred while generating access token: {}",
                    err
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let refresh_token_claim = jwt::Claims {
            exp: Utc::now().timestamp() as usize + jwt::REFRESH_TOKEN_EXPIRE_TIME,
            iss: "jizi".parse().unwrap(),
            typ: jwt::AccountType::User,
        };
        let refresh_token = match jwt::encode(&refresh_token_claim, &state.jwt_encoding_key) {
            Ok(token) => token,
            Err(err) => {
                warn!(
                    "internal server error occurred while generating refresh token: {}",
                    err
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        Ok(Json(LoginResponse {
            access_token,
            refresh_token,
        }))
    } else {
        debug!("401 Unauthorized(password)");
        Err(StatusCode::UNAUTHORIZED)
    }
}
