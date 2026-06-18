use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::domain::email_address::EmailAddress;
use crate::domain::password_credentials::PasswordCredentials;
use crate::domain::user::{User, UserStatus};
use crate::domain::user_id::UserId;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
        let users = self
            .users
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(users.get(&id).cloned())
    }

    async fn find_by_m_address(&self, m_address: &EmailAddress) -> Result<Option<User>, FindError> {
        let users = self
            .users
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(users.values().find(|u| u.m_address() == m_address).cloned())
    }

    async fn find_all(&self) -> Result<Vec<User>, FindError> {
        let users = self
            .users
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(users.values().cloned().collect())
    }

    async fn insert(&self, user: &User) -> Result<(), InsertError> {
        let mut users = self
            .users
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if users.contains_key(&user.id()) {
            return Err(InsertError::Conflict);
        }
        users.insert(user.id(), user.clone());
        Ok(())
    }

    async fn insert_in(
        &self,
        _tx: &mut MemoryTransaction,
        user: &User,
    ) -> Result<(), anyhow::Error> {
        self.insert(user).await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn update(&self, user: &User) -> Result<(), UpdateError> {
        let mut users = self
            .users
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if !users.contains_key(&user.id()) {
            return Err(UpdateError::NotFound);
        }
        users.insert(user.id(), user.clone());
        Ok(())
    }

    async fn update_in(
        &self,
        _tx: &mut MemoryTransaction,
        user: &User,
    ) -> Result<(), anyhow::Error> {
        self.update(user).await.map_err(|e| anyhow::anyhow!(e))
    }

    async fn rehash_password_in(
        &self,
        _tx: &mut MemoryTransaction,
        id: UserId,
        expected_old_phc: &str,
        new_phc: &str,
    ) -> Result<u64, anyhow::Error> {
        let mut users = self.users.write().map_err(|e| anyhow!(e.to_string()))?;
        let Some(user) = users.get(&id) else {
            return Ok(0);
        };
        // Active かつ phc が検証時のままのときだけ rehash する(changed_at は維持)。
        let UserStatus::Active {
            password_credentials,
        } = user.status()
        else {
            return Ok(0);
        };
        if password_credentials.phc() != expected_old_phc {
            return Ok(0);
        }
        let changed_at = *password_credentials.changed_at();
        let updated = User::restore(
            user.id(),
            *user.created_at(),
            *user.updated_at(),
            user.name().to_string(),
            user.m_address().clone(),
            UserStatus::Active {
                password_credentials: PasswordCredentials::restore(new_phc, changed_at)
                    .map_err(|e| anyhow!(e.to_string()))?,
            },
        );
        users.insert(id, updated);
        Ok(1)
    }

    async fn delete(&self, id: UserId) -> Result<(), DeleteError> {
        let mut users = self
            .users
            .write()
            .map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        users.remove(&id).ok_or(DeleteError::NotFound)?;
        Ok(())
    }

    async fn delete_in(
        &self,
        _tx: &mut MemoryTransaction,
        id: UserId,
    ) -> Result<(), anyhow::Error> {
        self.delete(id).await.map_err(|e| anyhow::anyhow!(e))
    }
}
