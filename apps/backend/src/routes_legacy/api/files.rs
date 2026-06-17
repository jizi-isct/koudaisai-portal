use crate::middlewares::CurrentUser;
use crate::routes_legacy::AppState;
use crate::util::AppResponse;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::extract::{ConnectInfo, Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v2/files")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/upload", post(post_files_upload))
        .route("/download", get(get_files_download))
}

#[derive(Deserialize, Serialize, Debug)]
struct PostFilesUploadPayload {
    file_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct PostFilesUploadResponse {
    presigned_url: String,
    key: String,
}

#[instrument(name = "POST /api/v2/files/upload", skip(state))]
async fn post_files_upload(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<PostFilesUploadPayload>,
) -> AppResponse {
    if matches!(current_user, CurrentUser::Admin(..) | CurrentUser::User(..)) {
        let key = format!("{}-{}", Uuid::new_v4(), payload.file_name);

        let presigned = state
            .s3_client
            .put_object()
            .bucket(state.s3_bucket.clone())
            .key(&key)
            .presigned(
                PresigningConfig::builder()
                    .expires_in(Duration::from_secs(60 * 10)) // 10 分
                    .build()?,
            )
            .await?;

        Ok((
            StatusCode::OK,
            Json(PostFilesUploadResponse {
                presigned_url: presigned.uri().to_string(),
                key: key.clone(),
            })
            .into_response(),
        ))
    } else {
        Ok((StatusCode::FORBIDDEN, "Forbidden".into_response()))
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct GetFilesDownloadParams {
    key: String,
    file_name: String,
    #[serde(default)]
    redirect: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct GetFilesDownloadResponse {
    presigned_url: String,
}

#[instrument(name = "GET /api/v2/files/download", skip(state))]
async fn get_files_download(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetFilesDownloadParams>,
) -> AppResponse {
    let presigned = state
        .s3_client
        .get_object()
        .bucket(state.s3_bucket.clone())
        .key(&params.key)
        .response_content_disposition(format!(r#"attachment; filename="{}""#, params.file_name))
        .presigned(
            PresigningConfig::builder()
                .expires_in(Duration::from_secs(60 * 10)) // 10 分
                .build()?,
        )
        .await?;

    if params.redirect {
        let mut header_map = HeaderMap::new();
        header_map.insert("Location", presigned.uri().to_string().parse()?);
        Ok((StatusCode::FOUND, header_map.into_response()))
    } else {
        Ok((
            StatusCode::OK,
            Json(GetFilesDownloadResponse {
                presigned_url: presigned.uri().to_string(),
            })
            .into_response(),
        ))
    }
}
