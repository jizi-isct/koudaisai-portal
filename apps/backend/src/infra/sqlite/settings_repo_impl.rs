use crate::application::error::{FindError, UpdateError};
use crate::application::ports::repositories::settings_repo::SettingsRepo;
use crate::domain::settings::Settings;
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

pub struct SqliteSettingsRepo {
    pool: SqlitePool,
}

impl SqliteSettingsRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepo for SqliteSettingsRepo {
    async fn get(&self) -> Result<Settings, FindError> {
        let row = sqlx::query(
            "SELECT show_occasions_on_portal, accept_correction_requests FROM settings WHERE singleton = 1",
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| FindError::InternalError(e.into()))?
            .ok_or_else(|| FindError::InternalError(anyhow::anyhow!("settings row is missing")))?;

        let show_occasions_on_portal = row
            .try_get::<i64, _>("show_occasions_on_portal")
            .map_err(|e| FindError::InternalError(e.into()))?;
        let accept_correction_requests = row
            .try_get::<i64, _>("accept_correction_requests")
            .map_err(|e| FindError::InternalError(e.into()))?;
        Ok(Settings::restore(
            show_occasions_on_portal != 0,
            accept_correction_requests != 0,
        ))
    }

    async fn save(&self, settings: &Settings) -> Result<(), UpdateError> {
        let show_occasions_on_portal = if settings.show_occasions_on_portal() {
            1_i64
        } else {
            0_i64
        };
        let accept_correction_requests = if settings.accept_correction_requests() {
            1_i64
        } else {
            0_i64
        };
        let result =
            sqlx::query(
                "UPDATE settings SET show_occasions_on_portal = ?, accept_correction_requests = ? WHERE singleton = 1",
            )
                .bind(show_occasions_on_portal)
                .bind(accept_correction_requests)
                .execute(&self.pool)
                .await
                .map_err(|e| UpdateError::InternalError(e.into()))?;

        if result.rows_affected() == 0 {
            return Err(UpdateError::NotFound);
        }
        Ok(())
    }
}
