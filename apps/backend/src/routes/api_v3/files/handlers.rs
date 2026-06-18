use super::dto::{FileDownloadResponse, FileUploadRequest, FileUploadResponse};
use axum::Json;
use axum::extract::Query;
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
pub async fn post_file_upload(Json(body): Json<FileUploadRequest>) -> PostFileUploadResponse {
    todo!()
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

#[http_response]
pub enum GetFileDownloadResponse {
    #[response(status = OK, description = "Returns the presigned download URL")]
    Ok(FileDownloadResponse),
    #[response(status = FOUND, description = "Redirect to the presigned URL (when redirect=true)")]
    Found,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Issue a presigned URL for downloading a file.",
    params(FileDownloadQuery),
    path = "/download",
    responses(GetFileDownloadResponse),
    tag = super::super::FILES_TAG
)]
pub async fn get_file_download(Query(query): Query<FileDownloadQuery>) -> GetFileDownloadResponse {
    todo!()
}
