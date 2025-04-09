use axum::response::{IntoResponse, Response};
use http::StatusCode;
use sea_orm::DbErr;
use std::fmt::{Display, Error};
use tracing::warn;

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
