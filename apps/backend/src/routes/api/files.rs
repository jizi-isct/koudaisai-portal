use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::extract::{ConnectInfo, State};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Extension, Json, Router};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v1/exhibitors")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new().route("/", post(post_files_upload))
}

#[derive(Deserialize, Serialize, Debug)]
struct PostFilesUploadPayload {
    file_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct PostFilesUploadResponse {
    presigned_url: String,
}

#[instrument(name = "POST /api/v1/files/upload", skip(state))]
async fn post_files_upload(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<PostFilesUploadPayload>,
) -> AppResponse {
    if let CurrentUser::Admin(claims) = current_user {
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
            })
            .into_response(),
        ))
    } else {
        Ok((StatusCode::FORBIDDEN, "Forbidden".into_response()))
    }
}
