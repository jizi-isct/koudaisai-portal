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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn change_title<C: Clock>(
        &mut self,
        new_title: String,
        clock: &C,
    ) -> Result<(), FactoryError> {
        if new_title.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Title is empty".to_string()));
        }
        self.title = new_title;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn emoji(&self) -> Option<String> {
        // Some(String)またはNone
        self.emoji.clone()
    }

    pub fn change_emoji<C: Clock>(
        &mut self,
        new_emoji: Option<String>,
        clock: &C,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::Membership;
    use crate::domain::user_id::UserId;
    use crate::sea_orm_entities::document;
    use aws_smithy_types::Document;
    use uuid::Uuid;

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    fn setup_document_category(clock: &MockClock) -> DocumentCategory {
        DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            clock,
        )
        .unwrap()
    }

    #[test]
    fn test_register_success() {
        let clock = MockClock { now: Utc::now() };
        let id = Uuid::new_v4();
        let title = "Test Category".to_string();
        let emoji = Some("📃".to_string());

        let document_category =
            DocumentCategory::register(id, title.clone(), emoji.clone(), &clock).unwrap();

        assert_eq!(document_category.id(), id);
        assert_eq!(document_category.title(), title);
        assert_eq!(document_category.emoji(), emoji);
        assert_eq!(document_category.created_at(), clock.now);
        assert_eq!(document_category.updated_at(), clock.now);
    }

    #[test]
    fn test_register_none_success() {
        let clock = MockClock { now: Utc::now() };
        let id = Uuid::new_v4();
        let title = "Test Category".to_string();
        let emoji = None;

        let document_category =
            DocumentCategory::register(id, title.clone(), emoji.clone(), &clock).unwrap();

        assert_eq!(document_category.id(), id);
        assert_eq!(document_category.title(), title);
        assert_eq!(document_category.emoji(), emoji);
        assert_eq!(document_category.created_at(), clock.now);
        assert_eq!(document_category.updated_at(), clock.now);
    }

    #[test]
    fn test_register_empty_name() {
        let clock = MockClock { now: Utc::now() };
        let id = Uuid::new_v4();
        let title = "  ".to_string();
        let emoji = Some("📃".to_string());

        let document_category =
            DocumentCategory::register(id, title.clone(), emoji.clone(), &clock);

        assert!(matches!(
            document_category,
            Err(FactoryError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_register_empty_emoji() {
        let clock = MockClock { now: Utc::now() };
        let id = Uuid::new_v4();
        let title = "Test Category".to_string();
        let emoji = Some("  ".to_string());

        let document_category =
            DocumentCategory::register(id, title.clone(), emoji.clone(), &clock);

        assert!(matches!(
            document_category,
            Err(FactoryError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_restore_success() {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let title = "Restored Category".to_string();
        let emoji = Some("📃".to_string());

        let document_category =
            DocumentCategory::restore(id, created_at, updated_at, title.clone(), emoji.clone())
                .unwrap();

        assert_eq!(document_category.id(), id);
        assert_eq!(document_category.title(), title);
        assert_eq!(document_category.emoji(), emoji);
        assert_eq!(document_category.created_at(), created_at);
        assert_eq!(document_category.updated_at(), updated_at);
    }

    #[test]
    fn test_change_title_success() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut document_category = setup_document_category(&clock);

        let new_title = "New Category Title".to_string();
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        let result = document_category.change_title(new_title.clone(), &clock_updated);
        assert!(result.is_ok());
        assert_eq!(document_category.title(), new_title);
        assert_eq!(document_category.updated_at(), update_time);
        assert_eq!(document_category.created_at(), initial_time);
    }

    #[test]
    fn test_change_title_empty() {
        let clock = MockClock { now: Utc::now() };
        let mut document_category = setup_document_category(&clock);

        let result = document_category.change_title("  ".to_string(), &clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }

    #[test]
    fn test_change_emoji_success() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut document_category = setup_document_category(&clock);

        let new_emoji = Some("✅️".to_string());
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        let result = document_category.change_emoji(new_emoji.clone(), &clock_updated);
        assert!(result.is_ok());
        assert_eq!(document_category.emoji(), new_emoji);
        assert_eq!(document_category.updated_at(), update_time);
        assert_eq!(document_category.created_at(), initial_time);
    }

    #[test]
    fn test_change_emoji_none_success() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut document_category = setup_document_category(&clock);

        let new_emoji = None;
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        let result = document_category.change_emoji(new_emoji.clone(), &clock_updated);
        assert!(result.is_ok());
        assert_eq!(document_category.emoji(), new_emoji);
        assert_eq!(document_category.updated_at(), update_time);
        assert_eq!(document_category.created_at(), initial_time);
    }

    #[test]
    fn test_change_emoji_empty() {
        let clock = MockClock { now: Utc::now() };
        let mut document_category = setup_document_category(&clock);

        let result = document_category.change_emoji(Some("  ".to_string()), &clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }
}
