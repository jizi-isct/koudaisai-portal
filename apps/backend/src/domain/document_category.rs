use crate::{application::ports::clock::Clock, domain::error::FactoryError};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct DocumentCategory {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub emoji: Option<String>,
}

impl DocumentCategory {
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

    pub fn change_title<C: Clock>(&mut self, new_title: String, clock: C) -> Result<(), FactoryError> {
        if new_title.trim().is_empty() {
            return Err(FactoryError::InvalidInput("title is empty".to_string()));
        }
        self.title = new_title;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn emoji(&self) -> Option<&str> { // Some(&str)またはNone
        self.emoji.as_deref()
    }

    pub fn change_emoji<C: Clock>(&mut self, new_emoji: Option<String>, clock: C) -> Result<(), FactoryError> {

        if let Some(new_emoji) = new_emoji { // emojiがNoneじゃなければ
            if new_emoji.trim().is_empty() {
                return Err(FactoryError::InvalidInput("emoji is empty".to_string())) // わざわざOption型なので、もし空文字列ならエラー返す
            }

            self.emoji = Some(new_emoji);
        } else {
            self.emoji = None;
        }

        self.updated_at = clock.now();

        Ok(())
    }


}