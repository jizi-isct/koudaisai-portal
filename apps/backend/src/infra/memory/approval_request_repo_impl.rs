use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::domain::approval_request::{ApprovalRequest};
use crate::domain::user_id::UserId;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::domain::approval_request_id::ApprovalRequestId;

pub struct MemoryApprovalRequestRepo {
    approval_requests: Arc<RwLock<HashMap<ApprovalRequestId, ApprovalRequest>>>,
}

impl MemoryApprovalRequestRepo {
    pub fn new() -> Self {
        Self {
            approval_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ApprovalRequestRepo<MemoryTransaction> for MemoryApprovalRequestRepo {
    async fn find_by_id(
        &self,
        id: ApprovalRequestId,
    ) -> Result<Option<ApprovalRequest>, FindError> {
        let approval_requests = self
            .approval_requests
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(approval_requests.get(&id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<ApprovalRequest>, FindError> {
        let approval_requests = self
            .approval_requests
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(approval_requests.values().cloned().collect())
    }

    async fn find_by_user_ids(
        &self,
        user_ids: &[UserId],
    ) -> Result<Vec<ApprovalRequest>, FindError> {
        let approval_requests = self
            .approval_requests
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(approval_requests
            .values()
            .filter(|r| user_ids.contains(&r.issued_by()))
            .cloned()
            .collect())
    }

    async fn insert(&self, request: &ApprovalRequest) -> Result<(), InsertError> {
        let mut approval_requests = self
            .approval_requests
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if approval_requests.contains_key(&request.id()) {
            return Err(InsertError::Conflict);
        }
        approval_requests.insert(request.id(), request.clone());
        Ok(())
    }

    async fn insert_in(
        &self,
        _tx: &mut MemoryTransaction,
        request: &ApprovalRequest,
    ) -> Result<(), anyhow::Error> {
        self.insert(request).await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn update(&self, request: &ApprovalRequest) -> Result<(), UpdateError> {
        let mut approval_requests = self
            .approval_requests
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if !approval_requests.contains_key(&request.id()) {
            return Err(UpdateError::NotFound);
        }
        approval_requests.insert(request.id(), request.clone());
        Ok(())
    }

    async fn update_in(
        &self,
        _tx: &mut MemoryTransaction,
        request: &ApprovalRequest,
    ) -> Result<(), anyhow::Error> {
        self.update(request).await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn delete(&self, id: ApprovalRequestId) -> Result<(), DeleteError> {
        let mut approval_requests = self
            .approval_requests
            .write()
            .map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        if approval_requests.remove(&id).is_none() {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }

    async fn delete_in(
        &self,
        _tx: &mut MemoryTransaction,
        id: ApprovalRequestId,
    ) -> Result<(), anyhow::Error> {
        self.delete(id).await.map_err(|e| anyhow::anyhow!(e))
    }
}
