use crate::domain::approval_request_id::ApprovalRequestId;
use crate::domain::error::FactoryError;
use crate::domain::notification::Notification;
use crate::domain::notification::NotificationType as DomainNotificationType;
use crate::domain::target_specifier::TargetSpecifier;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NotificationType {
    Markdown { title: String, content: String },
    ApprovalRequest { approval_request_id: Uuid },
}

impl From<&DomainNotificationType> for NotificationType {
    fn from(t: &DomainNotificationType) -> Self {
        match t {
            DomainNotificationType::Markdown { title, content } => NotificationType::Markdown {
                title: title.clone(),
                content: content.clone(),
            },
            DomainNotificationType::ApprovalRequest {
                approval_request_id,
            } => NotificationType::ApprovalRequest {
                approval_request_id: approval_request_id.as_uuid(),
            },
        }
    }
}

impl NotificationType {
    pub fn to_domain(&self) -> Result<DomainNotificationType, FactoryError> {
        match self {
            NotificationType::Markdown { title, content } => {
                DomainNotificationType::markdown(title.clone(), content.clone())
            }
            NotificationType::ApprovalRequest {
                approval_request_id,
            } => Ok(DomainNotificationType::approval_request(
                ApprovalRequestId::new(*approval_request_id),
            )),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct NotificationRead {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Option<Uuid>,
    #[schema(value_type = Vec<String>)]
    targets: Vec<TargetSpecifier>,
    #[serde(flatten)]
    r#type: NotificationType,
}

impl From<&Notification> for NotificationRead {
    fn from(n: &Notification) -> Self {
        NotificationRead {
            id: n.id().as_uuid(),
            created_at: *n.created_at(),
            updated_at: *n.updated_at(),
            created_by: n.created_by().map(|a| a.as_uuid()),
            targets: n.targets().to_vec(),
            r#type: n.notification_type().into(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct NotificationCreate {
    #[schema(value_type = Vec<String>)]
    pub targets: Vec<TargetSpecifier>,
    #[serde(flatten)]
    pub r#type: NotificationType,
}

#[derive(Deserialize, ToSchema)]
pub struct NotificationUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>)]
    pub targets: Option<Vec<TargetSpecifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
