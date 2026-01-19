use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::transaction::Transaction;
use crate::domain::group_id::GroupId;
use crate::domain::membership::Membership;
use crate::domain::user_id::UserId;

#[async_trait::async_trait]
pub trait MembershipRepo<Tx: Transaction> {
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Membership>, FindError>;
    async fn find_by_group_id(&self, group_id: GroupId) -> Result<Vec<Membership>, FindError>;
    async fn insert(&self, membership: Membership) -> Result<(), InsertError>;
    async fn insert_in(&self, tx: &mut Tx, membership: Membership) -> Result<(), InsertError>;
    async fn update(&self, membership: Membership) -> Result<(), UpdateError>;
    async fn update_in(&self, tx: &mut Tx, membership: Membership) -> Result<(), UpdateError>;
    async fn delete(&self, membership: Membership) -> Result<(), DeleteError>;
    async fn delete_in(&self, tx: &mut Tx, membership: Membership) -> Result<(), DeleteError>;
}