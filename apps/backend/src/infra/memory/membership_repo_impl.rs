use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::domain::group_id::GroupId;
use crate::domain::membership::Membership;
use crate::domain::user_id::UserId;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::{Arc, RwLock};

pub struct MemoryMembershipRepo {
    memberships: Arc<RwLock<Vec<Membership>>>,
}

impl MemoryMembershipRepo {
    pub fn new() -> Self {
        Self {
            memberships: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl MembershipRepo<MemoryTransaction> for MemoryMembershipRepo {
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Membership>, FindError> {
        let memberships = self
            .memberships
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(memberships
            .iter()
            .filter(|m| m.user_id() == user_id)
            .cloned()
            .collect())
    }

    async fn find_by_group_id(&self, group_id: GroupId) -> Result<Vec<Membership>, FindError> {
        let memberships = self
            .memberships
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(memberships
            .iter()
            .filter(|m| m.group_id() == group_id)
            .cloned()
            .collect())
    }

    async fn insert(&self, membership: Membership) -> Result<(), InsertError> {
        let mut memberships = self
            .memberships
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if memberships
            .iter()
            .any(|m| m.user_id() == membership.user_id() && m.group_id() == membership.group_id())
        {
            return Err(InsertError::Conflict);
        }
        memberships.push(membership);
        Ok(())
    }

    async fn insert_in(
        &self,
        _tx: &mut MemoryTransaction,
        membership: Membership,
    ) -> Result<(), InsertError> {
        self.insert(membership).await
    }

    async fn update(&self, membership: Membership) -> Result<(), UpdateError> {
        let mut memberships = self
            .memberships
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if let Some(m) = memberships
            .iter_mut()
            .find(|m| m.user_id() == membership.user_id() && m.group_id() == membership.group_id())
        {
            *m = membership;
            Ok(())
        } else {
            Err(UpdateError::NotFound)
        }
    }

    async fn update_in(
        &self,
        _tx: &mut MemoryTransaction,
        membership: Membership,
    ) -> Result<(), UpdateError> {
        self.update(membership).await
    }

    async fn delete(&self, membership: Membership) -> Result<(), DeleteError> {
        let mut memberships = self
            .memberships
            .write()
            .map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        let len_before = memberships.len();
        memberships.retain(|m| {
            !(m.user_id() == membership.user_id() && m.group_id() == membership.group_id())
        });
        if memberships.len() < len_before {
            Ok(())
        } else {
            Err(DeleteError::NotFound)
        }
    }

    async fn delete_in(
        &self,
        _tx: &mut MemoryTransaction,
        membership: Membership,
    ) -> Result<(), DeleteError> {
        self.delete(membership).await
    }
}
