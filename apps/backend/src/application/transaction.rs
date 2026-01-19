use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Failed to commit transaction: {0}")]
    Failed(String)
}

#[async_trait::async_trait]
pub trait Transaction {
    async fn begin(&mut self) -> Result<(), TransactionError>;
    async fn commit(&mut self) -> Result<(), TransactionError>;
    async fn rollback(&mut self) -> Result<(), TransactionError>;
}