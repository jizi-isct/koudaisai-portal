use crate::domain::approval_request::{
    ApprovalRequest, ApprovalRequestStatus as DomainApprovalRequestStatus,
    ApprovalRequestType as DomainApprovalRequestType,
};
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

impl From<&DomainApprovalRequestType> for ApprovalRequestType {
    fn from(t: &DomainApprovalRequestType) -> Self {
        match t {
            DomainApprovalRequestType::EditExhibitionInfo {
                description,
                icon_key,
            } => ApprovalRequestType::EditExhibitionInfo {
                description: description.clone(),
                icon_key: icon_key.clone(),
            },
        }
    }
}

impl From<&ApprovalRequestType> for DomainApprovalRequestType {
    fn from(t: &ApprovalRequestType) -> Self {
        match t {
            ApprovalRequestType::EditExhibitionInfo {
                description,
                icon_key,
            } => DomainApprovalRequestType::EditExhibitionInfo {
                description: description.clone(),
                icon_key: icon_key.clone(),
            },
        }
    }
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

impl From<&DomainApprovalRequestStatus> for ApprovalRequestStatus {
    fn from(s: &DomainApprovalRequestStatus) -> Self {
        match s {
            DomainApprovalRequestStatus::Pending => ApprovalRequestStatus::Pending,
            DomainApprovalRequestStatus::Approved {
                approved_by,
                approved_at,
                approval_reason,
            } => ApprovalRequestStatus::Approved {
                approved_by: approved_by.as_uuid(),
                approved_at: *approved_at,
                approval_reason: approval_reason.clone(),
            },
            DomainApprovalRequestStatus::Rejected {
                rejected_by,
                rejected_at,
                rejection_reason,
            } => ApprovalRequestStatus::Rejected {
                rejected_by: rejected_by.as_uuid(),
                rejected_at: *rejected_at,
                rejection_reason: rejection_reason.clone(),
            },
            DomainApprovalRequestStatus::Closed { closed_at } => ApprovalRequestStatus::Closed {
                closed_at: *closed_at,
            },
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ApprovalRequestRead {
    id: Uuid,
    issued_at: DateTime<Utc>,
    issued_by: Uuid,
    /// 申請の対象団体(`M-001` 形式)。承認時の企画情報の反映先でもある。
    group_id: String,
    issue_reason: String,
    #[serde(flatten)]
    r#type: ApprovalRequestType,
    #[serde(flatten)]
    status: ApprovalRequestStatus,
}

impl From<&ApprovalRequest> for ApprovalRequestRead {
    fn from(ar: &ApprovalRequest) -> Self {
        ApprovalRequestRead {
            id: ar.id().as_uuid(),
            issued_at: *ar.issued_at(),
            issued_by: ar.issued_by().into(),
            group_id: ar.group_id().to_string(),
            issue_reason: ar.issue_reason().to_string(),
            r#type: ar.request_type().into(),
            status: ar.status().into(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct ApprovalRequestCreate {
    #[serde(flatten)]
    pub r#type: ApprovalRequestType,
    /// 申請の対象団体(`M-001` 形式)。申請者は複数の団体に所属しうるため必須。
    pub group_id: String,
    pub issue_reason: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ApprovalActionBody {
    pub reason: Option<String>,
}
