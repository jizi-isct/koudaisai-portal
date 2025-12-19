use thiserror::Error;

#[derive(Debug, Error)]
pub enum FactoryError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}