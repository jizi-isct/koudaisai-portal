use crate::application::error::{FindError, UpdateError};
use crate::application::ports::repositories::settings_repo::SettingsRepo;
use crate::domain::settings::Settings;
use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::{Arc, RwLock};

/// 単一のグローバル設定を保持するテスト用リポジトリ。
pub struct MemorySettingsRepo {
    settings: Arc<RwLock<Settings>>,
}

impl Default for MemorySettingsRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl MemorySettingsRepo {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(RwLock::new(Settings::default())),
        }
    }
}

#[async_trait]
impl SettingsRepo for MemorySettingsRepo {
    async fn get(&self) -> Result<Settings, FindError> {
        self.settings
            .read()
            .map(|settings| settings.clone())
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))
    }

    async fn save(&self, settings: &Settings) -> Result<(), UpdateError> {
        let mut current = self
            .settings
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        *current = settings.clone();
        Ok(())
    }
}
