use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::transaction::Transaction;
use crate::domain::email_address::EmailAddress;
use crate::domain::user::User;
use crate::domain::user_id::UserId;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepo<Tx: Transaction> {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, FindError>;
    async fn find_by_m_address(&self, m_address: &EmailAddress) -> Result<Option<User>, FindError>;
    async fn find_all(&self) -> Result<Vec<User>, FindError>;
    async fn insert(&self, user: &User) -> Result<(), InsertError>;
    async fn insert_in(&self, tx: &mut Tx, user: &User) -> Result<(), anyhow::Error>;
    async fn update(&self, user: &User) -> Result<(), UpdateError>;
    async fn update_in(&self, tx: &mut Tx, user: &User) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: UserId) -> Result<(), DeleteError>;
    async fn delete_in(&self, tx: &mut Tx, id: UserId) -> Result<(), anyhow::Error>;
}
