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

#[derive(Deserialize, ToSchema)]
pub struct NotificationCreate {
    #[schema(value_type = Vec<String>)]
    targets: Vec<TargetSpecifier>,
    #[serde(flatten)]
    r#type: NotificationType,
}

#[derive(Deserialize, ToSchema)]
pub struct NotificationUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>)]
    targets: Option<Vec<TargetSpecifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}
