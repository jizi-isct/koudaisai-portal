use crate::entities::document;
use crate::entities::document::{
    DocumentCreate, DocumentRead, DocumentUpdate, DocumentWriteActiveModel,
};
use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::sea_orm_entities;
use crate::util::AppResponse;
use anyhow::{anyhow, Error};
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
use sea_orm::{ActiveModelTrait, DbErr, IntoActiveModel};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, instrument, warn};
use uuid::Uuid;

#[instrument(name = "init /api/v1/documents")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_documents).post(post_documents))
        .route(
            "/{document_id}",
            get(get_document)
                .patch(patch_document)
                .delete(delete_document),
        )
}

#[instrument(name = "GET /api/v1/documents", skip(state))]
async fn get_documents(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    // 権限確認
    let mut documents: Vec<DocumentRead> = match current_user {
        CurrentUser::None => {
            DocumentRead::find_from_required_one_of_scopes(&"none".to_string(), &state.db_conn)
                .await?
        }
        CurrentUser::User(claims) => {
            let user = UserRead::from(claims, &state.db_conn).await?;
            let exhibitor = user.get_exhibitor_read(&state.db_conn).await?;
            DocumentRead::find_from_required_one_of_scopes(
                &exhibitor.r#type.to_string(),
                &state.db_conn,
            )
            .await?
        }
        CurrentUser::Admin(..) => DocumentRead::get_all(&state.db_conn).await?,
    };

    documents.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));

    Ok((StatusCode::OK, Json(documents).into_response()))
}

#[instrument(name = "POST /api/v1/documents", skip(state))]
async fn post_documents(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(document): Json<DocumentCreate>,
) -> AppResponse {
    if let CurrentUser::Admin(claims) = current_user {
        let result = document
            .insert(Uuid::from_str(claims.subject().as_str())?, &state.db_conn)
            .await;
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

#[instrument(name = "GET /api/v1/documents/{document_id}", skip(state))]
async fn get_document(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(document_id): Path<Uuid>,
) -> AppResponse {
    let document: DocumentRead = DocumentRead::find_from_id(document_id, &state.db_conn).await?;
    match current_user {
        CurrentUser::None => {
            if document
                .required_one_of_scopes
                .contains(&"none".to_string())
            {
                Ok((StatusCode::OK, Json(document).into_response()))
            } else {
                Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
            }
        }
        CurrentUser::User(claims) => {
            let user = UserRead::from(claims, &state.db_conn).await?;
            let exhibitor = user.get_exhibitor_read(&state.db_conn).await?;
            if document
                .required_one_of_scopes
                .contains(&exhibitor.r#type.to_string())
            {
                Ok((StatusCode::OK, Json(document).into_response()))
            } else {
                Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
            }
        }
        CurrentUser::Admin(..) => Ok((StatusCode::OK, Json(document).into_response())),
    }
}

#[instrument(name = "PATCH /api/v1/documents/{document_id}", skip(state))]
async fn patch_document(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(document_id): Path<Uuid>,
    Json(document): Json<DocumentUpdate>,
) -> AppResponse {
    if let CurrentUser::Admin(..) = current_user {
        let result = document.update(document_id, &state.db_conn).await;
        match result {
            Ok(document) => Ok((StatusCode::CREATED, Json(document).into_response())),
            Err(err) => match err.downcast::<DbErr>() {
                Ok(DbErr::RecordNotUpdated) => {
                    Ok((StatusCode::NOT_FOUND, "Not Found".into_response()))
                }
                Ok(db_err) => {
                    warn!("Internal Server Error: {:?}", db_err);
                    Err(db_err.into())
                }
                Err(err) => {
                    warn!("Internal Server Error: {:?}", err);
                    Err(anyhow!("Internal Server Error: {:?}", err).into())
                }
            },
        }
    } else {
        Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
    }
}

#[instrument(name = "DELETE /api/v1/documents/{document_id}", skip(state))]
async fn delete_document(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(document_id): Path<Uuid>,
) -> AppResponse {
    if let CurrentUser::Admin(..) = current_user {
        match document::delete_document(document_id, &state.db_conn).await {
            Ok(affected_rows) => {
                if affected_rows == 0 {
                    Ok((StatusCode::NOT_FOUND, "Not Found".into_response()))
                } else {
                    Ok((StatusCode::OK, "OK".into_response()))
                }
            }
            Err(err) => {
                warn!("Internal Server Error: {:?}", err);
                Err(err.into())
            }
        }
    } else {
        Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
    }
}
