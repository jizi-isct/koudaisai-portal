use crate::routes::AppState;
use crate::util::jwt;
use axum::extract::{Request, State};
use axum::http;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use openidconnect::core::CoreUserInfoClaims;
use openidconnect::AccessToken;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tracing::log::{trace, warn};
use tracing::{debug, instrument};

#[derive(Clone, Debug)]
pub enum CurrentUser {
    None,
    User(jwt::Claims),
    Admin(CoreUserInfoClaims),
}

/// 認証ミドルウェア
/// ヘッダーに`Authorization: Bearer <token>`が含まれている場合、tokenの検証を行う。含まれていない場合はCurrentUser::Noneを`extensions`に挿入
#[instrument(name = "auth middleware", skip(state, req, next))]
pub async fn auth(State(state): State<Arc<AppState>>, mut req: Request, next: Next) -> Response {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    //ヘッダーからtokenを抽出
    let token = match auth_header {
        Some(auth_header) => match auth_header.strip_prefix("Bearer ") {
            Some(token) => token.to_string(),
            None => return StatusCode::UNAUTHORIZED.into_response(),
        },
        None => {
            req.extensions_mut().insert(CurrentUser::None);
            return next.run(req).await;
        }
    };

    // トークンのissを抽出
    let payload_base64 = match token.split(".").collect::<Vec<&str>>().get(1) {
        Some(payload) => payload.to_string(),
        None => {
            warn!("Authorization error: token was not jwt");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
    let mut payload = String::new();
    if let Err(err) = base64url::decode(&payload_base64)
        .unwrap()
        .as_slice()
        .read_to_string(&mut payload)
        .await
    {
        warn!("Authorization error banana: {:?}", err);
        return StatusCode::UNAUTHORIZED.into_response();
    }
    trace!("{}", payload);
    let insecure_token_data = match serde_json::from_str::<HashMap<String, Value>>(payload.as_str())
    {
        Ok(data) => data,
        Err(err) => {
            warn!("Authorization error money: {:?}", err);
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
    let iss = match insecure_token_data.get("iss") {
        Some(iss) => iss.to_string(),
        None => {
            warn!("Authorization error: claim iss not found in the jwt");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    }
    .strip_prefix("\"")
    .unwrap()
    .strip_suffix("\"")
    .unwrap()
    .to_string();

    // 自分自身が発行したトークンの場合：参加団体責任者アカウントとして処理
    // 他のissuerが発行したトークンの場合：adminアカウントとして処理
    trace!("{} {}", iss, state.jwt_manager.iss);
    if iss == state.jwt_manager.iss {
        trace!("token type: jizi jwt");
        let token = match state.jwt_manager.decode(&*token) {
            Ok(data) => data,
            Err(err) => {
                warn!("Authorization error: {:?}", err);
                return StatusCode::UNAUTHORIZED.into_response();
            }
        };
        if state.jwt_manager.is_access_token_valid(&token.claims) {
            req.extensions_mut().insert(CurrentUser::User(token.claims));
            next.run(req).await
        } else {
            debug!("Authorization error: access token invalid");
            StatusCode::UNAUTHORIZED.into_response()
        }
    } else {
        trace!("token type: oidc jwt");
        // user_infoを要求する。
        let access_token = AccessToken::new(token);
        let user_info = match state.oidc_client.user_info(access_token, None) {
            Ok(user_info) => user_info,
            Err(err) => {
                warn!("Authorization error: {:?}", err);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        let user_info: CoreUserInfoClaims = match user_info.request_async(&state.http_client).await
        {
            Ok(user_info) => user_info,
            Err(err) => {
                warn!("Authorization error: {:?}", err);
                return StatusCode::UNAUTHORIZED.into_response();
            }
        };
        req.extensions_mut().insert(CurrentUser::Admin(user_info));
        trace!("oidc auth ok");
        next.run(req).await
    }
}
