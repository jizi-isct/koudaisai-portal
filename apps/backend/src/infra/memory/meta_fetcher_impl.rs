use crate::application::error::ApplicationError;
use crate::application::ports::meta_fetcher::{MetaFetcher, PageMeta};
use async_trait::async_trait;

/// テスト用の [`MetaFetcher`] 実装。取得した URL をそのまま title に反映する。
pub struct MemoryMetaFetcher;

impl MemoryMetaFetcher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MemoryMetaFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MetaFetcher for MemoryMetaFetcher {
    async fn fetch(&self, url: &str) -> Result<PageMeta, ApplicationError> {
        Ok(PageMeta {
            title: Some(format!("title of {url}")),
            description: Some(format!("description of {url}")),
        })
    }
}
