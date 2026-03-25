use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::domain::document_category::DocumentCategory;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct MemoryDocumentCategoryRepo {
    document_categories: Arc<RwLock<HashMap<Uuid, DocumentCategory>>>,
}

impl MemoryDocumentCategoryRepo {
    pub fn new() -> Self {
        Self {
            document_categories: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DocumentCategoryRepo<MemoryTransaction> for MemoryDocumentCategoryRepo {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DocumentCategory>, FindError> {
        let categories = self
            .document_categories
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(categories.get(&id).map(|c| {
            DocumentCategory::restore(
                c.id(),
                c.created_at().clone(),
                c.updated_at().clone(),
                c.title().clone(),
                c.emoji().clone(),
            )
            .unwrap()
        }))
    }

    async fn find_all(&self) -> Result<Vec<DocumentCategory>, FindError> {
        let categories = self
            .document_categories
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(categories
            .values()
            .map(|c| {
                DocumentCategory::restore(
                    c.id(),
                    c.created_at().clone(),
                    c.updated_at().clone(),
                    c.title().clone(),
                    c.emoji().clone(),
                )
                .unwrap()
            })
            .collect())
    }

    async fn insert(&self, document_category: &DocumentCategory) -> Result<(), InsertError> {
        let mut categories = self
            .document_categories
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if categories.contains_key(&document_category.id()) {
            return Err(InsertError::Conflict);
        }
        categories.insert(document_category.id(), document_category.clone());
        Ok(())
    }

    async fn insert_in(
        &self,
        _tx: &mut MemoryTransaction,
        document_category: &DocumentCategory,
    ) -> Result<(), InsertError> {
        self.insert(document_category).await
    }

    async fn update(&self, document_category: &DocumentCategory) -> Result<(), UpdateError> {
        let mut categories = self
            .document_categories
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if !categories.contains_key(&document_category.id()) {
            return Err(UpdateError::NotFound);
        }
        categories.insert(document_category.id(), document_category.clone());
        Ok(())
    }

    async fn update_in(
        &self,
        _tx: &mut MemoryTransaction,
        document_category: &DocumentCategory,
    ) -> Result<(), UpdateError> {
        self.update(document_category).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), DeleteError> {
        let mut categories = self
            .document_categories
            .write()
            .map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        if categories.remove(&id).is_none() {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(&self, _tx: &mut MemoryTransaction, id: Uuid) -> Result<(), DeleteError> {
        self.delete(id).await
    }
}
