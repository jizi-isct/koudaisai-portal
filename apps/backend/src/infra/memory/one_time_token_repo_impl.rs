use crate::application::error::FindError;
use crate::application::ports::repositories::one_time_token_repo::OneTimeTokenRepo;
use crate::domain::one_time_token::{OneTimeToken, OneTimeTokenPurpose};
use crate::domain::one_time_token_id::OneTimeTokenId;
use crate::domain::user_id::UserId;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct MemoryOneTimeTokenRepo {
    tokens: Arc<RwLock<HashMap<OneTimeTokenId, OneTimeToken>>>,
}

impl MemoryOneTimeTokenRepo {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl OneTimeTokenRepo<MemoryTransaction> for MemoryOneTimeTokenRepo {
    async fn find_by_id(&self, id: OneTimeTokenId) -> Result<Option<OneTimeToken>, FindError> {
        let tokens = self
            .tokens
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(tokens.get(&id).cloned())
    }

    async fn insert_in(
        &self,
        _tx: &mut MemoryTransaction,
        token: &OneTimeToken,
    ) -> Result<(), anyhow::Error> {
        let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
        if tokens.contains_key(&token.id()) {
            return Err(anyhow!("one-time token id conflict: {}", token.id()));
        }
        tokens.insert(token.id(), token.clone());
        Ok(())
    }

    async fn consume_in(
        &self,
        _tx: &mut MemoryTransaction,
        id: OneTimeTokenId,
        now: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error> {
        let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
        let Some(token) = tokens.get_mut(&id) else {
            return Ok(0);
        };
        if token.consumed_at().is_none() && now < *token.expires_at() {
            // 在メモリモデルへ消費を反映(restore で書き戻す)。
            *token = OneTimeToken::restore(
                token.id(),
                token.user_id(),
                token.purpose(),
                token.token_hash().to_string(),
                *token.created_at(),
                *token.expires_at(),
                Some(now),
                token.m_address().cloned(),
            );
            Ok(1)
        } else {
            Ok(0)
        }
    }

    async fn invalidate_existing_for_in(
        &self,
        _tx: &mut MemoryTransaction,
        user_id: UserId,
        purpose: OneTimeTokenPurpose,
        now: DateTime<Utc>,
    ) -> Result<u64, anyhow::Error> {
        let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
        let mut count = 0;
        for token in tokens.values_mut() {
            if token.user_id() == user_id
                && token.purpose() == purpose
                && token.consumed_at().is_none()
            {
                *token = OneTimeToken::restore(
                    token.id(),
                    token.user_id(),
                    token.purpose(),
                    token.token_hash().to_string(),
                    *token.created_at(),
                    *token.expires_at(),
                    Some(now),
                    token.m_address().cloned(),
                );
                count += 1;
            }
        }
        Ok(count)
    }

    async fn delete_expired(&self, now: DateTime<Utc>) -> Result<u64, anyhow::Error> {
        let mut tokens = self.tokens.write().map_err(|e| anyhow!(e.to_string()))?;
        let before = tokens.len();
        tokens.retain(|_, t| now < *t.expires_at());
        Ok((before - tokens.len()) as u64)
    }
}
