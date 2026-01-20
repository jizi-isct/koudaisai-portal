use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use anyhow::anyhow;
use async_trait::async_trait;
use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::transaction::Transaction;
use crate::domain::email_address::EmailAddress;
use crate::domain::user::User;
use crate::domain::user_id::UserId;
use crate::infra::memory::transaction_impl::MemoryTransaction;

pub struct MemoryUserRepo {
    users: Arc<RwLock<HashMap<UserId, User>>>,
}

impl MemoryUserRepo {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepo<MemoryTransaction> for MemoryUserRepo {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, FindError> {
        let users = self.users.read().map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(users.get(&id).cloned())
    }

    async fn find_by_m_address(&self, m_address: &EmailAddress) -> Result<Option<User>, FindError> {
        let users = self.users.read().map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(users.values().find(|u| u.m_address() == m_address).cloned())
    }

    async fn find_all(&self) -> Result<Vec<User>, FindError> {
        let users = self.users.read().map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(users.values().cloned().collect())
    }

    async fn insert(&self, user: &User) -> Result<(), InsertError> {
        let mut users = self.users.write().map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if users.contains_key(&user.id()) {
            return Err(InsertError::Conflict);
        }
        users.insert(user.id(), user.clone());
        Ok(())
    }

    async fn insert_in(&self, _tx: &mut MemoryTransaction, user: &User) -> Result<(), anyhow::Error> {
        self.insert(user).await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn update(&self, user: &User) -> Result<(), UpdateError> {
        let mut users = self.users.write().map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if !users.contains_key(&user.id()) {
            return Err(UpdateError::NotFound);
        }
        users.insert(user.id(), user.clone());
        Ok(())
    }

    async fn update_in(&self, _tx: &mut MemoryTransaction, user: &User) -> Result<(), anyhow::Error> {
        self.update(user).await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn delete(&self, id: UserId) -> Result<(), DeleteError> {
        let mut users = self.users.write().map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        users.remove(&id).ok_or(DeleteError::NotFound)?;
        Ok(())
    }

    async fn delete_in(&self, _tx: &mut MemoryTransaction, id: UserId) -> Result<(), anyhow::Error> {
        self.delete(id).await.map_err(|e| anyhow::anyhow!(e))
    }
}
