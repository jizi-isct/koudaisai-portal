mod user;

// TODO(sqlx移行): UserRead は旧 entities 層。application 層へ再配線する
// use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes_legacy::AppState;
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
    let _ = (&state, &current_user);
    // TODO(sqlx移行): 下記の admin 判定 + DBアクセスを application 層へ再配線する
    // match current_user {
    //     CurrentUser::Admin(_) => {
    //         let users = UserRead::get_all(&state.db_conn).await?;
    //         Ok((StatusCode::OK, Json(users).into_response()))
    //     }
    //     _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    // }
    todo!("sqlx移行: application層へ再配線")
}
