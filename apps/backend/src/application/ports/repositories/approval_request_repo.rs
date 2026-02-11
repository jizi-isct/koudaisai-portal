use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::transaction::Transaction;
use crate::domain::approval_request::{ApprovalRequest, ApprovalRequestId};
use crate::domain::user_id::UserId;

#[async_trait::async_trait]
pub trait ApprovalRequestRepo<Tx: Transaction> {
    async fn find_by_id(&self, id: ApprovalRequestId) -> Result<Option<ApprovalRequest>, FindError>;
    async fn find_all(&self) -> Result<Vec<ApprovalRequest>, FindError>;
    async fn find_by_issued_by(&self, user_id: UserId) -> Result<Vec<ApprovalRequest>, FindError>;
    async fn insert(&self, request: &ApprovalRequest) -> Result<(), InsertError>;
    async fn insert_in(&self, tx: &mut Tx, request: &ApprovalRequest) -> Result<(), anyhow::Error>;
    async fn update(&self, request: &ApprovalRequest) -> Result<(), UpdateError>;
    async fn update_in(&self, tx: &mut Tx, request: &ApprovalRequest) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: ApprovalRequestId) -> Result<(), DeleteError>;
    async fn delete_in(&self, tx: &mut Tx, id: ApprovalRequestId) -> Result<(), anyhow::Error>;
}
