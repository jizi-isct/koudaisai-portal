use axum::response::{IntoResponse, Response};
use chrono::Duration;
use http::StatusCode;
use sea_orm::sqlx::Value;
use sea_orm::{ActiveValue, DbErr};
use tracing::warn;
use uuid::Uuid;

pub(crate) mod jwt;
pub mod oidc;
pub mod sha;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        warn!("Internal server error: {}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type AppResponse = Result<(StatusCode, Response), AppError>;

pub fn contains_uuid(tuple: (Uuid, Uuid, Uuid), target: Uuid) -> bool {
    tuple.0 == target || tuple.1 == target || tuple.2 == target
}

pub trait IntoActiveValue<T: Into<migration::Value>> {
    fn into_active_value(self) -> ActiveValue<T>;
}

impl<T: Into<migration::Value>> IntoActiveValue<T> for Option<T> {
    fn into_active_value(self) -> ActiveValue<T> {
        match self {
            Some(value) => ActiveValue::Set(value),
            None => ActiveValue::NotSet,
        }
    }
}

pub fn format_secs_ja_full(duration: i64) -> String {
    let hours = duration / 3600;
    let minutes = (duration % 3600) / 60;
    let seconds = duration % 60;

    let mut parts = Vec::new();
    if hours != 0 {
        parts.push(format!("{}時間", hours));
    }
    if minutes != 0 || hours != 0 {
        parts.push(format!("{}分", minutes));
    }
    parts.push(format!("{}秒", seconds));

    parts.join("")
}
