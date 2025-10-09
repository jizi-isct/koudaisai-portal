use crate::entities::document;
use crate::entities::document::{DocumentCreate, DocumentRead, DocumentUpdate};
use crate::entities::document_category::DocumentCategoryRead;
use crate::entities::target_specifier::TargetSpecifier;
use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use anyhow::anyhow;
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
use linked_hash_map::LinkedHashMap;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{instrument, warn};
use uuid::Uuid;

#[instrument(name = "init /api/v2/documents")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_documents).post(post_documents))
        .route("/by-category", get(get_documents_by_category))
        .route(
            "/{document_id}",
            get(get_document)
                .patch(patch_document)
                .delete(delete_document),
        )
}

#[instrument(name = "GET /api/v2/documents", skip(state))]
async fn get_documents(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    // 権限確認
    let mut documents: Vec<DocumentRead> = match current_user {
        CurrentUser::None => DocumentRead::get_all(&state.db_conn)
            .await?
            .into_iter()
            .filter(|document| {
                document
                    .targets
                    .iter()
                    .any(|target| matches!(target, TargetSpecifier::UserNologin))
            })
            .collect(),
        CurrentUser::User(claims) => {
            let user = UserRead::from(claims, &state.db_conn).await?;
            let mut documents = vec![];
            for document in DocumentRead::get_all(&state.db_conn).await? {
                for target in &document.targets {
                    if target.does_user_match(Some(&user), &state.db_conn).await? {
                        documents.push(document);
                        break;
                    }
                }
            }
            documents
        }
        CurrentUser::Admin(..) => DocumentRead::get_all(&state.db_conn).await?,
    };

    documents.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));

    Ok((StatusCode::OK, Json(documents).into_response()))
}

#[derive(Debug, Serialize)]
struct GetDocumentsByCategoryResponseEntry {
    category: Option<DocumentCategoryRead>,
    documents: Vec<DocumentRead>,
}

#[derive(Debug, Deserialize)]
struct GetDocumentsByCategoryQuery {
    include_empty_categories: Option<bool>,
}

#[instrument(name = "GET /api/v2/documents/by-category", skip(state))]
async fn get_documents_by_category(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Query(query): Query<GetDocumentsByCategoryQuery>,
) -> AppResponse {
    let include_empty_categories = query.include_empty_categories.unwrap_or(false);
    // 権限確認
    let mut documents: Vec<DocumentRead> = match current_user {
        CurrentUser::None => {
            let mut documents = vec![];
            for document in DocumentRead::get_all(&state.db_conn).await? {
                for target in &document.targets {
                    if matches!(target, TargetSpecifier::UserNologin) {
                        documents.push(document);
                        break;
                    }
                }
            }
            documents
        }
        CurrentUser::User(claims) => {
            let user = UserRead::from_claims(claims, &state.db_conn).await?;
            let mut documents = vec![];
            for document in DocumentRead::get_all(&state.db_conn).await? {
                for target in &document.targets {
                    if target.does_user_match(Some(&user), &state.db_conn).await? {
                        documents.push(document);
                        break;
                    }
                }
            }
            documents
        }
        CurrentUser::Admin(..) => DocumentRead::get_all(&state.db_conn).await?,
    };

    documents.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));

    let mut categories_document_map =
        LinkedHashMap::<Option<Uuid>, Vec<DocumentRead>>::from_iter([(None, vec![])]);
    if include_empty_categories {
        for category in DocumentCategoryRead::get_all(&state.db_conn).await? {
            categories_document_map.insert(Some(category.id), vec![]);
        }
    }

    for document in documents {
        categories_document_map
            .entry(document.category.clone())
            .and_modify(|v| v.push(document.clone()))
            .or_insert(vec![document]);
    }

    let mut result: Vec<GetDocumentsByCategoryResponseEntry> = vec![];
    for (k, v) in categories_document_map {
        result.push(match k {
            Some(uuid) => GetDocumentsByCategoryResponseEntry {
                category: Some(
                    DocumentCategoryRead::find_by_id(uuid, &state.db_conn)
                        .await?
                        .ok_or(anyhow!("document category not found"))?,
                ),
                documents: v,
            },
            None => GetDocumentsByCategoryResponseEntry {
                category: None,
                documents: v,
            },
        })
    }

    Ok((StatusCode::OK, Json(result).into_response()))
}

#[instrument(name = "POST /api/v2/documents", skip(state))]
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

#[instrument(name = "GET /api/v2/documents/{document_id}", skip(state))]
async fn get_document(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(document_id): Path<Uuid>,
) -> AppResponse {
    let document: DocumentRead = DocumentRead::find_from_id(document_id, &state.db_conn).await?;
    match current_user {
        CurrentUser::None => {
            for target in &document.targets {
                if matches!(target, TargetSpecifier::UserNologin) {
                    return Ok((StatusCode::OK, Json(document).into_response()));
                }
            }
            Ok((StatusCode::NOT_FOUND, "Not found.".into_response()))
        }
        CurrentUser::User(claims) => {
            let user = UserRead::from(claims, &state.db_conn).await?;
            for target in &document.targets {
                if target.does_user_match(Some(&user), &state.db_conn).await? {
                    return Ok((StatusCode::OK, Json(document).into_response()));
                }
            }
            Ok((StatusCode::NOT_FOUND, "Not found.".into_response()))
        }
        CurrentUser::Admin(..) => Ok((StatusCode::OK, Json(document).into_response())),
    }
}

#[instrument(name = "PATCH /api/v2/documents/{document_id}", skip(state))]
async fn patch_document(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(document_id): Path<Uuid>,
    Json(document): Json<DocumentUpdate>,
) -> AppResponse {
    if let CurrentUser::Admin(claims) = current_user {
        let result = document
            .update(
                document_id,
                Uuid::from_str(claims.subject().as_str())?,
                &state.db_conn,
            )
            .await;
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

#[instrument(name = "DELETE /api/v2/documents/{document_id}", skip(state))]
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
                    Ok((StatusCode::NO_CONTENT, ().into_response()))
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
