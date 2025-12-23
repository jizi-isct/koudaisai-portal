use thiserror::Error;

#[derive(Debug, Error)]
pub enum FactoryError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Domain error: {0}")]
    Domain(String),
}