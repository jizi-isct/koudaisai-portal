use anyhow::anyhow;
use thiserror::Error;
use crate::application::transaction::TransactionError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal error")]
    InternalError(#[from] anyhow::Error)
}

#[derive(Debug, Error)]
pub enum ApplicationOperationError<Op: OperationError> {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Operation failed: {0}")]
    OperationFailed(#[from] Op),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal error")]
    InternalError(anyhow::Error)
}

#[derive(Debug, Error)]
pub enum ApplicationSequentialOperationError<Op: OperationError> {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Operation failed: {0}")]
    OperationFailed(#[from] Op),
    #[error("Transaction error: {0}")]
    TransactionFailed(#[from] TransactionError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal error")]
    InternalError(anyhow::Error)
}

trait OperationError {}
impl OperationError for InsertError {}
impl OperationError for FindError {}
impl OperationError for UpdateError {}
impl OperationError for DeleteError {}

#[derive(Debug, Error)]
pub enum InsertError {
    #[error("Conflict")]
    Conflict,
    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error)
}

#[derive(Debug, Error)]
pub enum FindError {
    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error)
}

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Not found")]
    NotFound,
    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error)
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("Not found")]
    NotFound,
    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error)
}