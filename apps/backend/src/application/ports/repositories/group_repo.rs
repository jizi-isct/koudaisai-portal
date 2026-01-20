use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::transaction::Transaction;
use crate::domain::group::Group;
use crate::domain::group_id::GroupId;

#[async_trait::async_trait]
pub trait GroupRepo<Tx: Transaction> {
    async fn find_by_id(&self, id: GroupId) -> Result<Option<Group>, FindError>;
    async fn find_all(&self) -> Result<Vec<Group>, FindError>;
    async fn insert(&self, group: Group) -> Result<(), InsertError>;
    async fn insert_in(&self, tx: &mut Tx, group: Group) -> Result<(), InsertError>;
    async fn update(&self, group: Group) -> Result<(), UpdateError>;
    async fn update_in(&self, tx: &mut Tx, group: Group) -> Result<(), UpdateError>;
    async fn delete(&self, id: GroupId) -> Result<(), DeleteError>;
    async fn delete_in(&self, tx: &mut Tx, id: GroupId) -> Result<(), DeleteError>;
}