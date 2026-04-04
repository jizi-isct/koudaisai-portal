use crate::application::ports::clock::Clock;
use crate::domain::approval_request_id::ApprovalRequestId;
use crate::domain::error::{FactoryError, InvalidTransitionError};
use crate::domain::target_specifier::TargetSpecifier;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotificationId(Uuid);

impl NotificationId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Display for NotificationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
    target: Vec<TargetSpecifier>,
    notification_type: NotificationType,
}

impl Notification {
    pub fn create<C: Clock>(
        id: NotificationId,
        target: Vec<TargetSpecifier>,
        notification_type: NotificationType,
        created_by: Option<UserId>,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        Self::validate_target(&target)?;
        let now = clock.now();

        Ok(Self {
            id,
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by,
            target,
            notification_type,
        })
    }

    pub fn restore(
        id: NotificationId,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        created_by: Option<UserId>,
        updated_by: Option<UserId>,
        target: Vec<TargetSpecifier>,
        notification_type: NotificationType,
    ) -> Result<Self, FactoryError> {
        Self::validate_target(&target)?;

        Ok(Self {
            id,
            created_at,
            updated_at,
            created_by,
            updated_by,
            target,
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

    pub fn target(&self) -> &[TargetSpecifier] {
        &self.target
    }

    pub fn notification_type(&self) -> &NotificationType {
        &self.notification_type
    }

    pub fn update_target<C: Clock>(
        &mut self,
        target: Vec<TargetSpecifier>,
        updated_by: Option<UserId>,
        clock: &C,
    ) -> Result<(), FactoryError> {
        Self::validate_target(&target)?;
        self.target = target;
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

    fn validate_target(target: &[TargetSpecifier]) -> Result<(), FactoryError> {
        if target.is_empty() {
            return Err(FactoryError::InvalidInput(
                "Notification target is empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    #[test]
    fn create_fails_when_target_is_empty() {
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        };

        let result = Notification::create(
            NotificationId::generate(),
            vec![],
            NotificationType::approval_request(ApprovalRequestId::generate()),
            None,
            &clock,
        );

        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
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
}
