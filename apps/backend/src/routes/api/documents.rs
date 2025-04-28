use crate::entities::document::DocumentRead;
use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, State};
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use http::StatusCode;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api/v1/documents")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
}

#[instrument(name = "GET /api/v1/documents", skip(state))]
async fn get_documents(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    // 権限確認
    let documents: Vec<DocumentRead> = match current_user {
        CurrentUser::None => {
            DocumentRead::find_from_required_one_of_scopes("none", &state.db_conn).await?
        }
        CurrentUser::User(claims) => {
            let user = UserRead::from(claims, &state.db_conn).await?;
            let exhibitor = user.get_exhibitor_read(&state.db_conn).await?;
            DocumentRead::find_from_required_one_of_scopes(
                exhibitor.r#type.to_string(),
                &state.db_conn,
            )
            .await?
        }
        CurrentUser::Admin(claims) => DocumentRead::get_all(&state.db_conn).await?,
    };

    Ok((StatusCode::OK, Json(documents).into_response()))
}
