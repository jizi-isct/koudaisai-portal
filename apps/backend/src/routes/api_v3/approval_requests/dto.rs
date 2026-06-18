use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ApprovalRequestType {
    EditExhibitionInfo {
        description: Option<String>,
        icon_key: Option<String>,
    },
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ApprovalRequestStatus {
    Pending,
    Approved {
        approved_by: Uuid,
        approved_at: DateTime<Utc>,
        approval_reason: Option<String>,
    },
    Rejected {
        rejected_by: Uuid,
        rejected_at: DateTime<Utc>,
        rejection_reason: Option<String>,
    },
    Closed {
        closed_at: DateTime<Utc>,
    },
}

#[derive(Serialize, ToSchema)]
pub struct ApprovalRequestRead {
    id: Uuid,
    issued_at: DateTime<Utc>,
    issued_by: Uuid,
    issue_reason: String,
    #[serde(flatten)]
    r#type: ApprovalRequestType,
    #[serde(flatten)]
    status: ApprovalRequestStatus,
}

#[derive(Deserialize, ToSchema)]
pub struct ApprovalRequestCreate {
    #[serde(flatten)]
    r#type: ApprovalRequestType,
    issue_reason: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ApprovalActionBody {
    reason: Option<String>,
}
