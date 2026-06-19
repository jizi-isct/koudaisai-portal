use crate::application::error::ApplicationError;
use crate::application::ports::object_storage::ObjectStorage;
use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub struct MemoryObjectStorage {
    objects: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl Default for MemoryObjectStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryObjectStorage {
    pub fn new() -> Self {
        Self {
            objects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// テスト用にオブジェクト本体を格納する。
    pub fn put(&self, key: impl Into<String>, bytes: Vec<u8>) {
        self.objects.write().unwrap().insert(key.into(), bytes);
    }
}

#[async_trait]
impl ObjectStorage for MemoryObjectStorage {
    async fn presigned_upload_url(
        &self,
        key: &str,
        _expires_in: Duration,
    ) -> Result<String, ApplicationError> {
        Ok(format!("https://memory.local/upload/{}", key))
    }

    async fn presigned_download_url(
        &self,
        key: &str,
        file_name: &str,
        _expires_in: Duration,
    ) -> Result<String, ApplicationError> {
        Ok(format!(
            "https://memory.local/download/{}?file_name={}",
            key, file_name
        ))
    }

    async fn get_object(&self, key: &str) -> Result<Vec<u8>, ApplicationError> {
        self.objects
            .read()
            .unwrap()
            .get(key)
            .cloned()
            .ok_or_else(|| ApplicationError::InternalError(anyhow!("object not found: {key}")))
    }
}
