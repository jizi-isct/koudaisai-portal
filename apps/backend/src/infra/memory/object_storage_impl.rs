use crate::application::error::ApplicationError;
use crate::application::ports::object_storage::ObjectStorage;
use async_trait::async_trait;
use std::time::Duration;

pub struct MemoryObjectStorage;

impl MemoryObjectStorage {
    pub fn new() -> Self {
        MemoryObjectStorage
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
}
