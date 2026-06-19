use super::super::V3State;
use super::super::document_categories::DocumentCategoryRead;
use super::dto::{DocumentCreate, DocumentRead, DocumentUpdate, DocumentsByCategoryEntry};
use crate::application::error::{ApplicationOperationError, DeleteError, UpdateError};
use crate::domain::actor_ctx::ActorContext;
use axum::Json;
use axum::extract::{Path, Query, State};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct DocumentPath {
    id: Uuid,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DocumentQuery {
    category: Option<Uuid>,
}

#[http_response]
pub enum GetDocumentsResponse {
    #[response(status = OK)]
    Ok(Vec<DocumentRead>),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all documents visible to the caller.",
    params(DocumentQuery),
    path = "/",
    responses(GetDocumentsResponse),
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn get_documents(
    State(st): State<V3State>,
    actor: ActorContext,
    Query(query): Query<DocumentQuery>,
) -> GetDocumentsResponse {
    match st.app.document().get_all(&actor).await {
        Ok(docs) => GetDocumentsResponse::Ok(
            docs.iter()
                .filter(|d| match query.category {
                    Some(cat) => d.category() == Some(cat),
                    None => true,
                })
                .map(DocumentRead::from)
                .collect(),
        ),
        Err(_) => GetDocumentsResponse::InternalServerError,
    }
}

/// by-category のクエリパラメーター。
#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DocumentsByCategoryQuery {
    /// 真のとき、ドキュメントが無いカテゴリも空リストで含める。
    include_empty_categories: Option<bool>,
}

#[http_response]
pub enum GetDocumentsByCategoryResponse {
    #[response(status = OK)]
    Ok(Vec<DocumentsByCategoryEntry>),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get documents visible to the caller, grouped by category.",
    params(DocumentsByCategoryQuery),
    path = "/by-category",
    responses(GetDocumentsByCategoryResponse),
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn get_documents_by_category(
    State(st): State<V3State>,
    actor: ActorContext,
    Query(query): Query<DocumentsByCategoryQuery>,
) -> GetDocumentsByCategoryResponse {
    let Ok(by_category) = st.app.document().get_by_category(&actor).await else {
        return GetDocumentsByCategoryResponse::InternalServerError;
    };

    // カテゴリのメタ情報。閲覧権限が無い(非管理者)場合は空とし、id のみでグルーピングする。
    // created_at 昇順で並べたいので Vec<(Uuid, DocumentCategory)> も保持する。
    let categories = st
        .app
        .document_category()
        .get_all(&actor)
        .await
        .unwrap_or_default();
    let mut ordered_category_ids: Vec<Uuid> = categories.iter().map(|c| c.id()).collect();
    ordered_category_ids
        .sort_by_key(|id| categories.iter().find(|c| c.id() == *id).map(|c| c.created_at()));
    let lookup: HashMap<Uuid, DocumentCategoryRead> = categories
        .iter()
        .map(|c| (c.id(), DocumentCategoryRead::from(c)))
        .collect();

    let mut entries: Vec<DocumentsByCategoryEntry> = Vec::new();

    // カテゴリ付きのエントリ(カテゴリ created_at 昇順)。
    for cat_id in &ordered_category_ids {
        if let Some(docs) = by_category.get(&Some(*cat_id)) {
            entries.push(DocumentsByCategoryEntry {
                category: lookup.get(cat_id).cloned(),
                documents: docs.iter().map(DocumentRead::from).collect(),
            });
        } else if query.include_empty_categories.unwrap_or(false) {
            entries.push(DocumentsByCategoryEntry {
                category: lookup.get(cat_id).cloned(),
                documents: Vec::new(),
            });
        }
    }

    // lookup に無い id を持つカテゴリ付きドキュメント(権限で解決できなかった等)。
    for (cat_id_opt, docs) in &by_category {
        if let Some(cat_id) = cat_id_opt
            && !lookup.contains_key(cat_id) {
                entries.push(DocumentsByCategoryEntry {
                    category: None,
                    documents: docs.iter().map(DocumentRead::from).collect(),
                });
            }
    }

    // 未分類(None)は最後に。
    if let Some(docs) = by_category.get(&None) {
        entries.push(DocumentsByCategoryEntry {
            category: None,
            documents: docs.iter().map(DocumentRead::from).collect(),
        });
    }

    GetDocumentsByCategoryResponse::Ok(entries)
}

#[http_response]
pub enum PostDocumentResponse {
    #[response(status = CREATED)]
    Created(DocumentRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid document")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Create a document.",
    path = "/",
    responses(PostDocumentResponse),
    request_body = DocumentCreate,
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn post_document(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<DocumentCreate>,
) -> PostDocumentResponse {
    match st
        .app
        .document()
        .create(
            &actor,
            body.title,
            body.category,
            body.targets,
            body.format.into(),
        )
        .await
    {
        Ok(doc) => PostDocumentResponse::Created(DocumentRead::from(&doc)),
        Err(ApplicationOperationError::Unauthorized) => PostDocumentResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => PostDocumentResponse::UnprocessableEntity,
        Err(_) => PostDocumentResponse::InternalServerError,
    }
}

#[http_response]
pub enum GetDocumentResponse {
    #[response(status = OK, description = "Document found")]
    Ok(DocumentRead),
    #[response(status = NOT_FOUND, description = "Document not found")]
    NotFound,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get a document by id.",
    params(DocumentPath),
    path = "/{id}",
    responses(GetDocumentResponse),
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn get_document(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<DocumentPath>,
) -> GetDocumentResponse {
    match st.app.document().get_by_id(&actor, path.id).await {
        Ok(Some(doc)) => GetDocumentResponse::Ok(DocumentRead::from(&doc)),
        Ok(None) => GetDocumentResponse::NotFound,
        Err(_) => GetDocumentResponse::InternalServerError,
    }
}

#[http_response]
pub enum PatchDocumentResponse {
    #[response(status = OK, description = "Document updated")]
    Ok(DocumentRead),
    #[response(status = NOT_FOUND, description = "Document not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid document")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    patch,
    description = "Edit a document by id.",
    params(DocumentPath),
    path = "/{id}",
    responses(PatchDocumentResponse),
    request_body = DocumentUpdate,
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn patch_document(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<DocumentPath>,
    Json(body): Json<DocumentUpdate>,
) -> PatchDocumentResponse {
    match st
        .app
        .document()
        .update_document(
            &actor,
            path.id,
            body.title,
            body.category,
            body.format.map(Into::into),
            body.targets,
        )
        .await
    {
        Ok(()) => match st.app.document().get_by_id(&actor, path.id).await {
            Ok(Some(doc)) => PatchDocumentResponse::Ok(DocumentRead::from(&doc)),
            Ok(None) => PatchDocumentResponse::NotFound,
            Err(_) => PatchDocumentResponse::InternalServerError,
        },
        Err(ApplicationOperationError::Unauthorized) => PatchDocumentResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => {
            PatchDocumentResponse::UnprocessableEntity
        }
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PatchDocumentResponse::NotFound
        }
        Err(_) => PatchDocumentResponse::InternalServerError,
    }
}

#[http_response]
pub enum DeleteDocumentResponse {
    #[response(status = NO_CONTENT, description = "Document deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Document not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a document by id.",
    params(DocumentPath),
    path = "/{id}",
    responses(DeleteDocumentResponse),
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn delete_document(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<DocumentPath>,
) -> DeleteDocumentResponse {
    match st.app.document().delete_document(&actor, path.id).await {
        Ok(()) => DeleteDocumentResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteDocumentResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteDocumentResponse::NotFound
        }
        Err(_) => DeleteDocumentResponse::InternalServerError,
    }
}
