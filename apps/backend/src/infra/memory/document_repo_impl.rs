use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::document_repo::DocumentRepo;
use crate::domain::document::Document;
use crate::domain::target_specifier::TargetSpecifier;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct MemoryDocumentRepo {
    documents: Arc<RwLock<HashMap<Uuid, Document>>>,
}

impl MemoryDocumentRepo {
    pub fn new() -> Self {
        Self {
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DocumentRepo<MemoryTransaction> for MemoryDocumentRepo {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Document>, FindError> {
        let documents = self
            .documents
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(documents.get(&id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Document>, FindError> {
        let documents = self
            .documents
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(documents.values().cloned().collect())
    }

    async fn insert(&self, document: &Document) -> Result<(), InsertError> {
        let mut documents = self
            .documents
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if documents.contains_key(&document.id()) {
            return Err(InsertError::Conflict);
        }
        documents.insert(document.id(), document.clone());
        Ok(())
    }

    async fn insert_in(
        &self,
        _tx: &mut MemoryTransaction,
        document: &Document,
    ) -> Result<(), InsertError> {
        self.insert(document).await
    }

    async fn update(&self, document: &Document) -> Result<(), UpdateError> {
        let mut documents = self
            .documents
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if !documents.contains_key(&document.id()) {
            return Err(UpdateError::NotFound);
        }
        documents.insert(document.id(), document.clone());
        Ok(())
    }

    async fn update_in(
        &self,
        _tx: &mut MemoryTransaction,
        document: &Document,
    ) -> Result<(), UpdateError> {
        self.update(document).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), DeleteError> {
        let mut documents = self
            .documents
            .write()
            .map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        if documents.remove(&id).is_none() {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(&self, _tx: &mut MemoryTransaction, id: Uuid) -> Result<(), DeleteError> {
        self.delete(id).await
    }
}
