use crate::{application::ports::clock::Clock, domain::error::FactoryError};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DocumentCategory {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    emoji: Option<String>,
}

impl DocumentCategory {
    pub fn register<C: Clock>(
        id: Uuid,
        title: String,
        emoji: Option<String>,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        if title.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Title is empty".to_string()));
        }

        Ok(Self {
            id,
            created_at: clock.now(),
            updated_at: clock.now(),
            title,
            emoji,
        })
    }

    pub fn restore(
        id: Uuid,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        title: String,
        emoji: Option<String>,
    ) -> Result<Self, FactoryError> {
        if title.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Title is empty".to_string()));
        }

        Ok(Self {
            id,
            created_at,
            updated_at,
            title,
            emoji,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn change_title<C: Clock>(
        &mut self,
        new_title: String,
        clock: C,
    ) -> Result<(), FactoryError> {
        if new_title.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Title is empty".to_string()));
        }
        self.title = new_title;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn emoji(&self) -> Option<&str> {
        // Some(&str)またはNone
        self.emoji.as_deref()
    }

    pub fn change_emoji<C: Clock>(
        &mut self,
        new_emoji: Option<String>,
        clock: C,
    ) -> Result<(), FactoryError> {
        if let Some(new_emoji) = new_emoji {
            // emojiがNoneじゃなければ
            if new_emoji.trim().is_empty() {
                return Err(FactoryError::InvalidInput("Emoji is empty".to_string())); // わざわざOption型なので、もし空文字列ならエラー返す
            }

            self.emoji = Some(new_emoji);
        } else {
            self.emoji = None;
        }

        self.updated_at = clock.now();

        Ok(())
    }
}
