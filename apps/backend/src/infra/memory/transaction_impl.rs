use crate::application::transaction::{Transaction, TransactionError};
use async_trait::async_trait;

pub struct MemoryTransaction {
    is_active: bool,
}

impl MemoryTransaction {
    pub fn new() -> Self {
        Self { is_active: false }
    }
}

#[async_trait]
impl Transaction for MemoryTransaction {
    async fn begin(&mut self) -> Result<(), TransactionError> {
        self.is_active = true;
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), TransactionError> {
        self.is_active = false;
        Ok(())
    }
}
