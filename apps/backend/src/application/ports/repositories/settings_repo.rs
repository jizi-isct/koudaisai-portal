use crate::application::error::{FindError, UpdateError};
use crate::domain::settings::Settings;

#[async_trait::async_trait]
pub trait SettingsRepo {
    async fn get(&self) -> Result<Settings, FindError>;
    async fn save(&self, settings: &Settings) -> Result<(), UpdateError>;
}
