use crate::entities::document::{DocumentActiveModel, DocumentRead, DocumentWrite};
use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::sea_orm_entities;
use crate::util::AppResponse;
use anyhow::anyhow;
use axum::extract::{ConnectInfo, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
use sea_orm::{ActiveModelTrait, DbErr, IntoActiveModel};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, instrument, warn};

#[instrument(name = "init /api/v1/documents")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_documents).post(post_documents))
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
        CurrentUser::Admin(..) => DocumentRead::get_all(&state.db_conn).await?,
    };

    Ok((StatusCode::OK, Json(documents).into_response()))
}

#[instrument(name = "POST /api/v1/documents", skip(state))]
async fn post_documents(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(document): Json<DocumentWrite>,
) -> AppResponse {
    if let CurrentUser::Admin(..) = current_user {
        let result = document.insert(&state.db_conn).await;
        match result {
            Ok(document) => Ok((StatusCode::CREATED, Json(document).into_response())),
            Err(DbErr::RecordNotInserted) => Ok((StatusCode::CONFLICT, "Conflict".into_response())),
            Err(err) => {
                warn!("Internal Server Error: {:?}", err);
                Err(err.into())
            }
        }
    } else {
        Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
    }
}
