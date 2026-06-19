use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// アップロード用 presigned URL のリクエストボディ。
#[derive(Deserialize, ToSchema)]
pub struct FileUploadRequest {
    pub file_name: String,
}

/// アップロード用に発行した presigned URL とストレージキー。
#[derive(Serialize, ToSchema)]
pub struct FileUploadResponse {
    pub presigned_url: String,
    pub key: String,
}

/// ダウンロード用に発行した presigned URL。
#[derive(Serialize, ToSchema)]
pub struct FileDownloadResponse {
    pub presigned_url: String,
}
