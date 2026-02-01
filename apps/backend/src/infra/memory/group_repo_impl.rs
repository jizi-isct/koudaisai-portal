use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::domain::group::Group;
use crate::domain::group_id::GroupId;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct MemoryGroupRepo {
    groups: Arc<RwLock<HashMap<GroupId, Group>>>,
}

impl MemoryGroupRepo {
    pub fn new() -> Self {
        Self {
            groups: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl GroupRepo<MemoryTransaction> for MemoryGroupRepo {
    async fn find_by_id(&self, id: GroupId) -> Result<Option<Group>, FindError> {
        let groups = self
            .groups
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(groups.get(&id).map(|g| {
            Group::restore(
                g.id(),
                g.created_at(),
                g.updated_at(),
                g.name().to_string(),
                g.r#type().clone(),
            )
            .unwrap()
        }))
    }

    async fn find_all(&self) -> Result<Vec<Group>, FindError> {
        let groups = self
            .groups
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(groups
            .values()
            .map(|g| {
                Group::restore(
                    g.id(),
                    g.created_at(),
                    g.updated_at(),
                    g.name().to_string(),
                    g.r#type().clone(),
                )
                .unwrap()
            })
            .collect())
    }

    async fn insert(&self, group: Group) -> Result<(), InsertError> {
        let mut groups = self
            .groups
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if groups.contains_key(&group.id()) {
            return Err(InsertError::Conflict);
        }
        groups.insert(group.id(), group);
        Ok(())
    }

    async fn insert_in(
        &self,
        _tx: &mut MemoryTransaction,
        group: Group,
    ) -> Result<(), InsertError> {
        self.insert(group).await
    }

    async fn update(&self, group: Group) -> Result<(), UpdateError> {
        let mut groups = self
            .groups
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if !groups.contains_key(&group.id()) {
            return Err(UpdateError::NotFound);
        }
        groups.insert(group.id(), group);
        Ok(())
    }

    async fn update_in(
        &self,
        _tx: &mut MemoryTransaction,
        group: Group,
    ) -> Result<(), UpdateError> {
        self.update(group).await
    }

    async fn delete(&self, id: GroupId) -> Result<(), DeleteError> {
        let mut groups = self
            .groups
            .write()
            .map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        groups.remove(&id).ok_or(DeleteError::NotFound)?;
        Ok(())
    }

    async fn delete_in(&self, _tx: &mut MemoryTransaction, id: GroupId) -> Result<(), DeleteError> {
        self.delete(id).await
    }
}
