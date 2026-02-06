use uuid::Uuid;
use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::transaction::Transaction;

use crate::domain::document_category::DocumentCategory;

#[async_trait::async_trait]
pub trait DocumentCategoryRepo<Tx: Transaction> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DocumentCategory>, FindError>;
    async fn find_all(&self) -> Result<Vec<DocumentCategory>, FindError>;

    async fn insert(&self, id: Uuid) -> Result<DocumentCategory, InsertError>;
    async fn insert_in(&self, tx: &mut Tx, id: &Uuid) -> Result<(), anyhow::Error>;
    
    async fn update(&self, id: Uuid) -> Result<Option<DocumentCategory>, UpdateError>;
    async fn update_in(&self, tx: &mut Tx, id: &Uuid) -> Result<(), anyhow::Error>;

    async fn delete(&self, id: Uuid) -> Result<u64, DeleteError>;
    async fn delete_in(&self, tx: &mut Tx, id: Uuid) -> Result<(), anyhow::Error>;
}