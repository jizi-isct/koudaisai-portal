use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::domain::form::Form;
use crate::domain::form_id::FormId;

#[async_trait::async_trait]
pub trait FormRepo {
    async fn find_by_id(&self, id: FormId) -> Result<Option<Form>, FindError>;
    async fn find_all(&self) -> Result<Vec<Form>, FindError>;
    async fn insert(&self, group: Form) -> Result<(), InsertError>;
    async fn update(&self, group: Form) -> Result<(), UpdateError>;
    async fn delete(&self, id: FormId) -> Result<(), DeleteError>;
}