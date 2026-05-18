use crate::application::ports::clock::Clock;
use crate::domain::approval_request_id::ApprovalRequestId;
use crate::domain::error::{FactoryError, InvalidTransitionError};
use crate::domain::notification_id::NotificationId;
use crate::domain::target_specifier::TargetSpecifier;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationType {
    Markdown {
        title: String,
        content: String,
    },
    ApprovalRequest {
        approval_request_id: ApprovalRequestId,
    },
}

impl NotificationType {
    pub fn markdown(title: String, content: String) -> Result<Self, FactoryError> {
        if title.trim().is_empty() {
            return Err(FactoryError::InvalidInput(
                "Notification title is empty".to_string(),
            ));
        }
        if content.trim().is_empty() {
            return Err(FactoryError::InvalidInput(
                "Notification content is empty".to_string(),
            ));
        }

        Ok(Self::Markdown { title, content })
    }

    pub fn approval_request(approval_request_id: ApprovalRequestId) -> Self {
        Self::ApprovalRequest {
            approval_request_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    id: NotificationId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Option<UserId>,
    updated_by: Option<UserId>,
    targets: Vec<TargetSpecifier>,
    notification_type: NotificationType,
}

impl Notification {
    pub fn create<C: Clock>(
        id: NotificationId,
        targets: Vec<TargetSpecifier>,
        notification_type: NotificationType,
        created_by: Option<UserId>,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        let now = clock.now();

        Ok(Self {
            id,
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by,
            targets,
            notification_type,
        })
    }

    pub fn restore(
        id: NotificationId,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        created_by: Option<UserId>,
        updated_by: Option<UserId>,
        targets: Vec<TargetSpecifier>,
        notification_type: NotificationType,
    ) -> Result<Self, FactoryError> {
        Ok(Self {
            id,
            created_at,
            updated_at,
            created_by,
            updated_by,
            targets,
            notification_type,
        })
    }

    pub fn id(&self) -> NotificationId {
        self.id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn created_by(&self) -> Option<UserId> {
        self.created_by
    }

    pub fn updated_by(&self) -> Option<UserId> {
        self.updated_by
    }

    pub fn targets(&self) -> &[TargetSpecifier] {
        &self.targets
    }

    pub fn notification_type(&self) -> &NotificationType {
        &self.notification_type
    }

    pub fn update_target<C: Clock>(
        &mut self,
        targets: Vec<TargetSpecifier>,
        updated_by: Option<UserId>,
        clock: &C,
    ) -> Result<(), FactoryError> {
        self.targets = targets;
        self.updated_by = updated_by;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn update_markdown<C: Clock>(
        &mut self,
        title: String,
        content: String,
        updated_by: Option<UserId>,
        clock: &C,
    ) -> Result<(), InvalidTransitionError> {
        match self.notification_type {
            NotificationType::Markdown { .. } => {
                self.notification_type = NotificationType::Markdown { title, content };
                self.updated_by = updated_by;
                self.updated_at = clock.now();
                Ok(())
            }
            NotificationType::ApprovalRequest { .. } => Err(InvalidTransitionError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use uuid::Uuid;

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    #[test]
    fn create_allows_empty_target() {
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        };

        let result = Notification::create(
            NotificationId::generate(),
            vec![],
            NotificationType::approval_request(ApprovalRequestId::generate()),
            None,
            &clock,
        )
        .unwrap();

        assert!(result.targets().is_empty());
    }

    #[test]
    fn create_with_non_empty_target_sets_fields() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let clock = MockClock { now };
        let created_by = Some(UserId::new(Uuid::new_v4()));
        let notification_type =
            NotificationType::markdown("title".to_string(), "body".to_string()).unwrap();

        let notification = Notification::create(
            NotificationId::generate(),
            vec![TargetSpecifier::UserNologin],
            notification_type.clone(),
            created_by,
            &clock,
        )
        .unwrap();

        assert_eq!(notification.targets(), &[TargetSpecifier::UserNologin]);
        assert_eq!(notification.created_at(), &now);
        assert_eq!(notification.updated_at(), &now);
        assert_eq!(notification.created_by(), created_by);
        assert_eq!(notification.updated_by(), created_by);
        assert_eq!(notification.notification_type(), &notification_type);
    }

    #[test]
    fn restore_allows_empty_target() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let notification = Notification::restore(
            NotificationId::generate(),
            now,
            now,
            None,
            None,
            vec![],
            NotificationType::approval_request(ApprovalRequestId::generate()),
        )
        .unwrap();

        assert!(notification.targets().is_empty());
    }

    #[test]
    fn markdown_constructor_validates_title_and_content() {
        assert!(matches!(
            NotificationType::markdown("".to_string(), "x".to_string()),
            Err(FactoryError::InvalidInput(_))
        ));

        assert!(matches!(
            NotificationType::markdown("title".to_string(), "   ".to_string()),
            Err(FactoryError::InvalidInput(_))
        ));
    }

    #[test]
    fn update_markdown_on_approval_request_is_invalid_transition() {
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        };

        let mut notification = Notification::create(
            NotificationId::generate(),
            vec![TargetSpecifier::UserNologin],
            NotificationType::approval_request(ApprovalRequestId::generate()),
            None,
            &clock,
        )
        .unwrap();

        let result = notification.update_markdown(
            "new title".to_string(),
            "new body".to_string(),
            None,
            &clock,
        );

        assert!(matches!(result, Err(InvalidTransitionError)));
    }

    #[test]
    fn update_target_allows_empty_target() {
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        };

        let mut notification = Notification::create(
            NotificationId::generate(),
            vec![TargetSpecifier::UserNologin],
            NotificationType::approval_request(ApprovalRequestId::generate()),
            None,
            &clock,
        )
        .unwrap();

        notification.update_target(vec![], None, &clock).unwrap();
        assert!(notification.targets().is_empty());
    }

    #[test]
    fn update_target_with_non_empty_target_updates_values() {
        let created_at = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap();
        let create_clock = MockClock { now: created_at };
        let update_clock = MockClock { now: updated_at };
        let updater = Some(UserId::new(Uuid::new_v4()));

        let mut notification = Notification::create(
            NotificationId::generate(),
            vec![],
            NotificationType::approval_request(ApprovalRequestId::generate()),
            None,
            &create_clock,
        )
        .unwrap();

        notification
            .update_target(vec![TargetSpecifier::UserNologin], updater, &update_clock)
            .unwrap();

        assert_eq!(notification.targets(), &[TargetSpecifier::UserNologin]);
        assert_eq!(notification.updated_by(), updater);
        assert_eq!(notification.updated_at(), &updated_at);
    }

    #[test]
    fn update_markdown_on_markdown_notification_updates_content() {
        let created_at = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap();
        let create_clock = MockClock { now: created_at };
        let update_clock = MockClock { now: updated_at };
        let updater = Some(UserId::new(Uuid::new_v4()));

        let mut notification = Notification::create(
            NotificationId::generate(),
            vec![TargetSpecifier::UserNologin],
            NotificationType::markdown("old title".to_string(), "old body".to_string()).unwrap(),
            None,
            &create_clock,
        )
        .unwrap();

        notification
            .update_markdown(
                "new title".to_string(),
                "new body".to_string(),
                updater,
                &update_clock,
            )
            .unwrap();

        assert_eq!(notification.updated_by(), updater);
        assert_eq!(notification.updated_at(), &updated_at);
        assert_eq!(
            notification.notification_type(),
            &NotificationType::Markdown {
                title: "new title".to_string(),
                content: "new body".to_string(),
            }
        );
    }
}
