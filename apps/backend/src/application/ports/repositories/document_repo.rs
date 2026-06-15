use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};

use crate::application::transaction::Transaction;

use uuid::Uuid;

use crate::domain::document::Document;

#[async_trait::async_trait]
pub trait DocumentRepo<Tx: Transaction> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Document>, FindError>;
    async fn find_all(&self) -> Result<Vec<Document>, FindError>;

    async fn insert(&self, document: &Document) -> Result<(), InsertError>;
    async fn insert_in(&self, tx: &mut Tx, document: &Document) -> Result<(), InsertError>;

    async fn update(&self, document: &Document) -> Result<(), UpdateError>;
    async fn update_in(&self, tx: &mut Tx, document: &Document) -> Result<(), UpdateError>;

    async fn delete(&self, id: Uuid) -> Result<(), DeleteError>;
    async fn delete_in(&self, tx: &mut Tx, id: Uuid) -> Result<(), DeleteError>;
}
