use super::super::V3State;
use super::dto::{FileDownloadResponse, FileUploadRequest, FileUploadResponse};
use crate::application::error::ApplicationError;
use crate::domain::actor_ctx::ActorContext;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;

#[http_response]
pub enum PostFileUploadResponse {
    #[response(status = OK, description = "Returns the presigned upload URL and key")]
    Ok(FileUploadResponse),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Issue a presigned URL for uploading a file.",
    path = "/upload",
    responses(PostFileUploadResponse),
    request_body = FileUploadRequest,
    tag = super::super::FILES_TAG
)]
pub async fn post_file_upload(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<FileUploadRequest>,
) -> PostFileUploadResponse {
    match st.app.file().request_upload(&actor, &body.file_name).await {
        Ok(ticket) => PostFileUploadResponse::Ok(FileUploadResponse {
            presigned_url: ticket.presigned_url,
            key: ticket.key,
        }),
        Err(ApplicationError::Unauthorized) => PostFileUploadResponse::Forbidden,
        Err(_) => PostFileUploadResponse::InternalServerError,
    }
}

/// ダウンロード用のクエリパラメーター。
#[derive(Deserialize, IntoParams)]
pub struct FileDownloadQuery {
    key: String,
    file_name: String,
    /// 真のとき presigned URL へ 302 リダイレクトする。
    #[serde(default)]
    redirect: bool,
}

// `redirect=true` のとき Location ヘッダ付き 302 を返すため、戻り値は [`Response`]。
// OpenAPI は `#[utoipa::path]` の `responses(...)` で記述する。
#[utoipa::path(
    get,
    description = "Issue a presigned URL for downloading a file.",
    params(FileDownloadQuery),
    path = "/download",
    responses(
        (status = OK, description = "Returns the presigned download URL", body = FileDownloadResponse),
        (status = FOUND, description = "Redirect to the presigned URL (when redirect=true)"),
        (status = FORBIDDEN, description = "Forbidden"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    ),
    tag = super::super::FILES_TAG
)]
pub async fn get_file_download(
    State(st): State<V3State>,
    actor: ActorContext,
    Query(query): Query<FileDownloadQuery>,
) -> Response {
    match st
        .app
        .file()
        .request_download(&actor, &query.key, &query.file_name)
        .await
    {
        Ok(url) => {
            if query.redirect {
                match HeaderValue::from_str(&url) {
                    Ok(loc) => (StatusCode::FOUND, [(header::LOCATION, loc)]).into_response(),
                    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                }
            } else {
                (
                    StatusCode::OK,
                    Json(FileDownloadResponse { presigned_url: url }),
                )
                    .into_response()
            }
        }
        Err(ApplicationError::Unauthorized) => StatusCode::FORBIDDEN.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
