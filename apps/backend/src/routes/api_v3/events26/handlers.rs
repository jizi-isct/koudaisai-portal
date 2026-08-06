//! 企画情報API(events26)の `/admin/v1` 中継。
//!
//! 企画スキーマの正本は events26 側の OpenAPI 仕様で、ポータルは中身を解釈せず
//! そのまま中継する。DTO は設けず、生成型 [`Project`] をリクエスト/レスポンス
//! 双方でそのまま使う(生成テンプレートで `utoipa::ToSchema` を derive しているため、
//! ポータル側の OpenAPI にもスキーマがそのまま載る)。

use super::super::V3State;
use crate::application::error::{ApplicationOperationError, DeleteError, InsertError, UpdateError};
use crate::application::events26::Events26App;
use crate::application::ports::events26_api::UpdateIconError;
use crate::domain::actor_ctx::ActorContext;
use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::http::header::CONTENT_TYPE;
use events26_api::models::Project;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;

/// アイコンとして受け付けるメディアタイプ。events26 の仕様に合わせる。
/// ここで弾いておくと、非対応形式を上流まで運ばずに済む。
const ICON_CONTENT_TYPES: [&str; 5] = [
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
    "image/heic",
];

/// アイコンの最大サイズ。events26 の上限(20MB)に合わせる。
/// axum の既定ボディ上限は 2MB なので、アイコンのルートにだけこの値を適用する。
pub const ICON_MAX_BYTES: usize = 20 * 1024 * 1024;

#[derive(Deserialize, IntoParams)]
pub struct ProjectPath {
    /// 企画情報API 側の企画 ID。
    project_id: String,
}

#[http_response]
pub enum PostProjectResponse {
    #[response(status = CREATED, description = "Project created")]
    Created(Project),
    #[response(status = CONFLICT, description = "Project id already exists")]
    Conflict,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid project")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Create a project on the events26 API. The id is supplied by the caller.",
    path = "/projects",
    responses(PostProjectResponse),
    request_body = Project,
    tag = super::super::EVENTS26_TAG
)]
pub async fn post_project(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<Project>,
) -> PostProjectResponse {
    match Events26App::new(st.events26.as_ref())
        .create_project(&actor, &body)
        .await
    {
        Ok(project) => PostProjectResponse::Created(project),
        Err(ApplicationOperationError::Unauthorized) => PostProjectResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => PostProjectResponse::UnprocessableEntity,
        Err(ApplicationOperationError::OperationFailed(InsertError::Conflict)) => {
            PostProjectResponse::Conflict
        }
        Err(_) => PostProjectResponse::InternalServerError,
    }
}

#[http_response]
pub enum PutProjectResponse {
    #[response(status = OK, description = "Project replaced")]
    Ok(Project),
    #[response(status = NOT_FOUND, description = "Project not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid project")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    put,
    description = "Replace a project on the events26 API. Tags and occasions are replaced wholesale, not merged.",
    params(ProjectPath),
    path = "/projects/{project_id}",
    responses(PutProjectResponse),
    request_body = Project,
    tag = super::super::EVENTS26_TAG
)]
pub async fn put_project(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ProjectPath>,
    Json(body): Json<Project>,
) -> PutProjectResponse {
    match Events26App::new(st.events26.as_ref())
        .update_project(&actor, &path.project_id, &body)
        .await
    {
        Ok(project) => PutProjectResponse::Ok(project),
        Err(ApplicationOperationError::Unauthorized) => PutProjectResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => PutProjectResponse::UnprocessableEntity,
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PutProjectResponse::NotFound
        }
        Err(_) => PutProjectResponse::InternalServerError,
    }
}

#[http_response]
pub enum DeleteProjectResponse {
    #[response(status = NO_CONTENT, description = "Project deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Project not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a project on the events26 API. Tags and occasions are removed with it.",
    params(ProjectPath),
    path = "/projects/{project_id}",
    responses(DeleteProjectResponse),
    tag = super::super::EVENTS26_TAG
)]
pub async fn delete_project(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ProjectPath>,
) -> DeleteProjectResponse {
    match Events26App::new(st.events26.as_ref())
        .delete_project(&actor, &path.project_id)
        .await
    {
        Ok(()) => DeleteProjectResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteProjectResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteProjectResponse::NotFound
        }
        Err(_) => DeleteProjectResponse::InternalServerError,
    }
}

#[http_response]
pub enum PutProjectIconResponse {
    #[response(status = NO_CONTENT, description = "Icon stored")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Project not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNSUPPORTED_MEDIA_TYPE, description = "Unsupported image format")]
    UnsupportedMediaType,
    #[response(
        status = UNPROCESSABLE_ENTITY,
        description = "Image rejected (empty, too large, or not square)"
    )]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    put,
    description = "Replace a project icon on the events26 API. The body is the raw image; \
                   it must be square and at most 20MB.",
    params(ProjectPath),
    path = "/projects/{project_id}/icon",
    responses(PutProjectIconResponse),
    request_body(
        content = String,
        content_type = "application/octet-stream",
        description = "Raw image bytes. Send the actual format in Content-Type \
                       (image/png, image/jpeg, image/gif, image/webp, image/heic)."
    ),
    tag = super::super::EVENTS26_TAG
)]
pub async fn put_project_icon(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ProjectPath>,
    headers: HeaderMap,
    body: Bytes,
) -> PutProjectIconResponse {
    // events26 は Content-Type で形式を判定するため、指定が無い/対応外なら中継しない。
    // パラメータ付き(`image/png; charset=...`)でも通るよう `;` の手前だけ見る。
    let Some(content_type) = headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(';').next().unwrap_or_default().trim())
        .filter(|value| ICON_CONTENT_TYPES.contains(value))
    else {
        return PutProjectIconResponse::UnsupportedMediaType;
    };

    match Events26App::new(st.events26.as_ref())
        .update_project_icon(&actor, &path.project_id, content_type, body.to_vec())
        .await
    {
        Ok(()) => PutProjectIconResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => PutProjectIconResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(UpdateIconError::NotFound)) => {
            PutProjectIconResponse::NotFound
        }
        Err(ApplicationOperationError::OperationFailed(UpdateIconError::InvalidImage(_))) => {
            PutProjectIconResponse::UnprocessableEntity
        }
        Err(_) => PutProjectIconResponse::InternalServerError,
    }
}

#[http_response]
pub enum DeleteProjectIconResponse {
    #[response(status = NO_CONTENT, description = "Icon deleted")]
    NoContent,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a project icon on the events26 API. Succeeds even if no icon is set.",
    params(ProjectPath),
    path = "/projects/{project_id}/icon",
    responses(DeleteProjectIconResponse),
    tag = super::super::EVENTS26_TAG
)]
pub async fn delete_project_icon(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ProjectPath>,
) -> DeleteProjectIconResponse {
    match Events26App::new(st.events26.as_ref())
        .delete_project_icon(&actor, &path.project_id)
        .await
    {
        Ok(()) => DeleteProjectIconResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteProjectIconResponse::Forbidden,
        Err(_) => DeleteProjectIconResponse::InternalServerError,
    }
}
