use crate::application::error::{FindError, UpdateError};
use crate::domain::settings::Settings;

#[async_trait::async_trait]
pub trait SettingsRepo<Tx: Transaction> {
    async fn get(&self) -> Result<Settings, FindError>;
    async fn save(&self, user: &User) -> Result<(), UpdateError>;
}
