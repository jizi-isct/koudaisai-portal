use axum::response::{IntoResponse, Response};
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
