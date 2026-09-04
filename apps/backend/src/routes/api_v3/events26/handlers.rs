//! 企画情報API(events26)の `/admin/v1` 中継。
//!
//! 企画スキーマの正本は events26 側の OpenAPI 仕様で、ポータルは中身を解釈せず
//! そのまま中継する。DTO は設けず、生成型 [`Project`] をリクエスト/レスポンス
//! 双方でそのまま使う(生成テンプレートで `utoipa::ToSchema` を derive しているため、
//! ポータル側の OpenAPI にもスキーマがそのまま載る)。

use super::super::V3State;
use crate::application::error::{ApplicationOperationError, DeleteError, InsertError, UpdateError};
use crate::application::ports::events26_api::{UpdateIconError, UpdateMenuError};
use crate::domain::actor_ctx::ActorContext;
use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::http::header::CONTENT_TYPE;
use events26_api::models::{GetProjectDetails200ResponseMenu, Project};
use serde::Deserialize;
use tracing::warn;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;

/// 失敗の詳細を呼び出し側へ返す文字列にする。
///
/// events26 が返したステータスと本文は [`internal_error`] が anyhow のメッセージへ
/// 載せているので、それをそのまま渡す。中継先の検証(400 = リクエストボディが不正 など)は
/// 本文にしか理由が出ず、握り潰すと管理画面から原因が分からなくなるため。
/// 載るのは events26 の応答のみで、資格情報は含まない。
///
/// [`internal_error`]: crate::infra::events26_api_client
fn detail<E: std::fmt::Display>(operation: &str, error: E) -> String {
    let detail = error.to_string();
    warn!("events26 relay {operation} failed: {detail}");
    detail
}

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
    #[response(
        status = UNPROCESSABLE_ENTITY,
        description = "Invalid project. The body carries the reason."
    )]
    UnprocessableEntity(String),
    #[response(
        status = INTERNAL_SERVER_ERROR,
        description = "Internal server error. The body carries the upstream status and message."
    )]
    InternalServerError(String),
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
    match st.app.events26().create_project(&actor, &body).await {
        Ok(project) => PostProjectResponse::Created(project),
        Err(ApplicationOperationError::Unauthorized) => PostProjectResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(reason)) => {
            PostProjectResponse::UnprocessableEntity(detail("create_project", reason))
        }
        Err(ApplicationOperationError::OperationFailed(InsertError::Conflict)) => {
            PostProjectResponse::Conflict
        }
        Err(error) => PostProjectResponse::InternalServerError(detail("create_project", error)),
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
    #[response(
        status = UNPROCESSABLE_ENTITY,
        description = "Invalid project. The body carries the reason."
    )]
    UnprocessableEntity(String),
    #[response(
        status = INTERNAL_SERVER_ERROR,
        description = "Internal server error. The body carries the upstream status and message."
    )]
    InternalServerError(String),
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
    match st
        .app
        .events26()
        .update_project(&actor, &path.project_id, &body)
        .await
    {
        Ok(project) => PutProjectResponse::Ok(project),
        Err(ApplicationOperationError::Unauthorized) => PutProjectResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(reason)) => {
            PutProjectResponse::UnprocessableEntity(detail("update_project", reason))
        }
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PutProjectResponse::NotFound
        }
        Err(error) => PutProjectResponse::InternalServerError(detail("update_project", error)),
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
    #[response(
        status = INTERNAL_SERVER_ERROR,
        description = "Internal server error. The body carries the upstream status and message."
    )]
    InternalServerError(String),
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
    match st
        .app
        .events26()
        .delete_project(&actor, &path.project_id)
        .await
    {
        Ok(()) => DeleteProjectResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteProjectResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteProjectResponse::NotFound
        }
        Err(error) => DeleteProjectResponse::InternalServerError(detail("delete_project", error)),
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
    #[response(
        status = UNSUPPORTED_MEDIA_TYPE,
        description = "Unsupported image format. The body carries the received Content-Type."
    )]
    UnsupportedMediaType(String),
    #[response(
        status = UNPROCESSABLE_ENTITY,
        description = "Image rejected (empty, too large, or not square). The body carries the reason."
    )]
    UnprocessableEntity(String),
    #[response(
        status = INTERNAL_SERVER_ERROR,
        description = "Internal server error. The body carries the upstream status and message."
    )]
    InternalServerError(String),
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
    let received = headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let Some(content_type) = Some(received.split(';').next().unwrap_or_default().trim())
        .filter(|value| ICON_CONTENT_TYPES.contains(value))
    else {
        return PutProjectIconResponse::UnsupportedMediaType(detail(
            "update_project_icon",
            format!(
                "unsupported Content-Type {received:?} (expected one of {})",
                ICON_CONTENT_TYPES.join(", ")
            ),
        ));
    };

    match st
        .app
        .events26()
        .update_project_icon(&actor, &path.project_id, content_type, body.to_vec())
        .await
    {
        Ok(()) => PutProjectIconResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => PutProjectIconResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(UpdateIconError::NotFound)) => {
            PutProjectIconResponse::NotFound
        }
        Err(ApplicationOperationError::OperationFailed(UpdateIconError::InvalidImage(reason))) => {
            PutProjectIconResponse::UnprocessableEntity(detail("update_project_icon", reason))
        }
        Err(error) => {
            PutProjectIconResponse::InternalServerError(detail("update_project_icon", error))
        }
    }
}

#[http_response]
pub enum DeleteProjectIconResponse {
    #[response(status = NO_CONTENT, description = "Icon deleted")]
    NoContent,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(
        status = INTERNAL_SERVER_ERROR,
        description = "Internal server error. The body carries the upstream status and message."
    )]
    InternalServerError(String),
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
    match st
        .app
        .events26()
        .delete_project_icon(&actor, &path.project_id)
        .await
    {
        Ok(()) => DeleteProjectIconResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteProjectIconResponse::Forbidden,
        Err(error) => {
            DeleteProjectIconResponse::InternalServerError(detail("delete_project_icon", error))
        }
    }
}

#[http_response]
pub enum PutOwnProjectMenuResponse {
    #[response(status = NO_CONTENT, description = "Menu stored")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Project not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(
        status = UNPROCESSABLE_ENTITY,
        description = "Invalid menu. The body carries the upstream reason."
    )]
    UnprocessableEntity(String),
    #[response(
        status = INTERNAL_SERVER_ERROR,
        description = "Internal server error. The body carries the upstream status and message."
    )]
    InternalServerError(String),
}

#[utoipa::path(
    put,
    description = "Store the menu of the project belonging to the signed-in group. The project id is derived from the user's membership.",
    path = "/projects/us/menu",
    responses(PutOwnProjectMenuResponse),
    request_body = GetProjectDetails200ResponseMenu,
    tag = super::super::EVENTS26_TAG
)]
pub async fn put_own_project_menu(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<GetProjectDetails200ResponseMenu>,
) -> PutOwnProjectMenuResponse {
    match st
        .app
        .events26()
        .update_own_project_menu(&actor, &body)
        .await
    {
        Ok(()) => PutOwnProjectMenuResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => PutOwnProjectMenuResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(UpdateMenuError::NotFound)) => {
            PutOwnProjectMenuResponse::NotFound
        }
        Err(ApplicationOperationError::OperationFailed(UpdateMenuError::InvalidMenu(reason))) => {
            PutOwnProjectMenuResponse::UnprocessableEntity(detail("update_project_menu", reason))
        }
        Err(error) => {
            PutOwnProjectMenuResponse::InternalServerError(detail("update_project_menu", error))
        }
    }
}

#[http_response]
pub enum DeleteOwnProjectMenuResponse {
    #[response(status = NO_CONTENT, description = "Menu deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Project not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(
        status = INTERNAL_SERVER_ERROR,
        description = "Internal server error. The body carries the upstream status and message."
    )]
    InternalServerError(String),
}

#[utoipa::path(
    delete,
    description = "Delete the menu of the project belonging to the signed-in group. The project id is derived from the user's membership.",
    path = "/projects/us/menu",
    responses(DeleteOwnProjectMenuResponse),
    tag = super::super::EVENTS26_TAG
)]
pub async fn delete_own_project_menu(
    State(st): State<V3State>,
    actor: ActorContext,
) -> DeleteOwnProjectMenuResponse {
    match st.app.events26().delete_own_project_menu(&actor).await {
        Ok(()) => DeleteOwnProjectMenuResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteOwnProjectMenuResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteOwnProjectMenuResponse::NotFound
        }
        Err(error) => {
            DeleteOwnProjectMenuResponse::InternalServerError(detail("delete_project_menu", error))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 開催場所が空欄の企画は、`place` を持たない JSON として中継されること。
    ///
    /// events26 の `place` は enum であって nullable ではないため、`null` や空文字で
    /// 送ると 400(リクエストボディが不正)になる。ポータルは受け取った JSON を
    /// 生成型へ入れて送り直すので、この往復でキーが復活しないことを確かめる。
    #[test]
    fn omits_place_when_absent() {
        let body = serde_json::json!({
            "id": "S-001",
            "groupName": "団体",
            "projectName": "企画",
            "description": "説明",
            "isChildFriendly": true,
            "isRecommended": false,
            "type": "stage",
            "occasions": [{
                "timeRange": {
                    "start": { "date": 1, "hour": 10, "minute": 0 },
                    "end": { "date": 1, "hour": 11, "minute": 30 }
                }
            }]
        });

        let project: Project = serde_json::from_value(body).expect("should deserialize");
        let relayed = serde_json::to_value(&project).expect("should serialize");
        let occasion = &relayed["occasions"][0];

        assert!(
            occasion.get("place").is_none(),
            "place should be omitted, got {occasion}"
        );
    }

    /// `place` が `null` で届いた場合も、中継時にはキーごと落ちること。
    #[test]
    fn omits_place_when_null() {
        let body = serde_json::json!({
            "id": "S-002",
            "groupName": "団体",
            "projectName": "企画",
            "description": "説明",
            "isChildFriendly": false,
            "isRecommended": false,
            "type": "stage",
            "occasions": [{
                "place": null,
                "timeRange": {
                    "start": { "date": 2, "hour": 9, "minute": 0 },
                    "end": { "date": 2, "hour": 9, "minute": 45 }
                }
            }]
        });

        let project: Project = serde_json::from_value(body).expect("should deserialize");
        let relayed = serde_json::to_value(&project).expect("should serialize");
        let occasion = &relayed["occasions"][0];

        assert!(
            occasion.get("place").is_none(),
            "place should be omitted, got {occasion}"
        );
    }
}
