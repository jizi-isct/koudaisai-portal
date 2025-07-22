mod user;

use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api/v2/users")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .nest("/{user_id}", user::init_router())
}

#[instrument(name = "GET /api/v2/users", skip(state, current_user))]
async fn get_users(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            let users = UserRead::get_all(&state.db_conn).await?;
            Ok((StatusCode::OK, Json(users).into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
}
