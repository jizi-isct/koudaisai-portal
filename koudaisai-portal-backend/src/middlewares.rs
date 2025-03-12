use crate::routes::AppState;
use crate::util::jwt;
use axum::extract::{Request, State};
use axum::http;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum UserId {
    None,
    User(Uuid),
    Admin(Uuid),
}

pub async fn auth(State(state): State<Arc<AppState>>, mut req: Request, next: Next) -> Response {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = if auth_header.is_some() {
        match auth_header.unwrap().strip_prefix("Bearer ") {
            Some(token) => token.to_string(),
            None => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    } else {
        let cookies = CookieJar::from_headers(req.headers());
        let token = cookies.get("jizi_token");
        match token {
            Some(token) => token.value().to_string(),
            None => {
                req.extensions_mut().insert(UserId::None);
                return next.run(req).await;
            }
        }
    };

    match jwt::decode(&*token, &state.jwt_decoding_key) {
        Ok(jwt) => {
            if jwt.claims.exp < Utc::now().timestamp() as usize {
                req.extensions_mut().insert(UserId::None);
                next.run(req).await
            } else {
                let uid = match Uuid::parse_str(&*jwt.claims.sub) {
                    Ok(uid) => uid,
                    Err(_) => {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                };
                let user = match &*jwt.claims.role {
                    "user" => UserId::User(uid),
                    "admin" => UserId::Admin(uid),
                    _ => {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                };
                req.extensions_mut().insert(user);
                next.run(req).await
            }
        }
        Err(_) => {
            req.extensions_mut().insert(UserId::None);
            next.run(req).await
        }
    }
}
