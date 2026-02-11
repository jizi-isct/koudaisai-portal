use crate::application::ports::clock::Clock;
use crate::domain::error::{FactoryError, InvalidTransitionError};
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 承認申請ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApprovalRequestId(Uuid);

impl ApprovalRequestId {
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

/// 承認申請のタイプ
#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalRequestType {
    EditExhibitionInfo {
        description: Option<String>,
        icon_key: Option<String>,
    },
}

/// 承認申請のステータス
#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalRequestStatus {
    /// 申請中（審査待ち）
    Pending,
    /// 承認済み
    Approved {
        approved_by: UserId,
        approved_at: DateTime<Utc>,
        approval_reason: Option<String>,
    },
    /// 却下済み
    Rejected {
        rejected_by: UserId,
        rejected_at: DateTime<Utc>,
        rejection_reason: Option<String>,
    },
    /// クローズ（申請者によりキャンセル）
    Closed { closed_at: DateTime<Utc> },
}

/// 承認申請ドメインモデル
#[derive(Debug, Clone, PartialEq)]
pub struct ApprovalRequest {
    id: ApprovalRequestId,
    issued_at: DateTime<Utc>,
    issued_by: UserId,
    request_type: ApprovalRequestType,
    status: ApprovalRequestStatus,
    issue_reason: String,
}

impl ApprovalRequest {
    /// 新規申請を作成
    pub fn create<C: Clock>(
        id: ApprovalRequestId,
        issued_by: UserId,
        request_type: ApprovalRequestType,
        issue_reason: String,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        if issue_reason.trim().is_empty() {
            return Err(FactoryError::InvalidInput(
                "Issue reason is empty".to_string(),
            ));
        }

        Ok(Self {
            id,
            issued_at: clock.now(),
            issued_by,
            request_type,
            status: ApprovalRequestStatus::Pending,
            issue_reason,
        })
    }

    /// DBからの復元用コンストラクタ
    pub fn restore(
        id: ApprovalRequestId,
        issued_at: DateTime<Utc>,
        issued_by: UserId,
        request_type: ApprovalRequestType,
        status: ApprovalRequestStatus,
        issue_reason: String,
    ) -> Self {
        Self {
            id,
            issued_at,
            issued_by,
            request_type,
            status,
            issue_reason,
        }
    }

    // Getters
    pub fn id(&self) -> ApprovalRequestId {
        self.id
    }

    pub fn issued_at(&self) -> &DateTime<Utc> {
        &self.issued_at
    }

    pub fn issued_by(&self) -> UserId {
        self.issued_by
    }

    pub fn request_type(&self) -> &ApprovalRequestType {
        &self.request_type
    }

    pub fn status(&self) -> &ApprovalRequestStatus {
        &self.status
    }

    pub fn issue_reason(&self) -> &str {
        &self.issue_reason
    }

    /// 承認する（Pending → Approved）
    pub fn approve<C: Clock>(
        &mut self,
        approved_by: UserId,
        approval_reason: Option<String>,
        clock: &C,
    ) -> Result<(), InvalidTransitionError> {
        match &self.status {
            ApprovalRequestStatus::Pending => {
                self.status = ApprovalRequestStatus::Approved {
                    approved_by,
                    approved_at: clock.now(),
                    approval_reason,
                };
                Ok(())
            }
            _ => Err(InvalidTransitionError),
        }
    }

    /// 却下する（Pending → Rejected）
    pub fn reject<C: Clock>(
        &mut self,
        rejected_by: UserId,
        rejection_reason: Option<String>,
        clock: &C,
    ) -> Result<(), InvalidTransitionError> {
        match &self.status {
            ApprovalRequestStatus::Pending => {
                self.status = ApprovalRequestStatus::Rejected {
                    rejected_by,
                    rejected_at: clock.now(),
                    rejection_reason,
                };
                Ok(())
            }
            _ => Err(InvalidTransitionError),
        }
    }

    /// クローズする（申請者によるキャンセル、Pending → Closed）
    pub fn close<C: Clock>(&mut self, clock: &C) -> Result<(), InvalidTransitionError> {
        match &self.status {
            ApprovalRequestStatus::Pending => {
                self.status = ApprovalRequestStatus::Closed {
                    closed_at: clock.now(),
                };
                Ok(())
            }
            _ => Err(InvalidTransitionError),
        }
    }

    /// 申請がPending状態かどうか
    pub fn is_pending(&self) -> bool {
        matches!(&self.status, ApprovalRequestStatus::Pending)
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

    impl Clock for &MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    #[test]
    fn test_create_approval_request_success() {
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        };
        let id = ApprovalRequestId::generate();
        let issued_by = UserId::new(Uuid::new_v4());
        let request_type = ApprovalRequestType::EditExhibitionInfo {
            description: Some("New description".to_string()),
            icon_key: None,
        };

        let request = ApprovalRequest::create(
            id,
            issued_by,
            request_type.clone(),
            "Need to update info".to_string(),
            &clock,
        )
        .unwrap();

        assert_eq!(request.id(), id);
        assert_eq!(request.issued_by(), issued_by);
        assert!(request.is_pending());
    }

    #[test]
    fn test_create_with_empty_reason_fails() {
        let clock = MockClock { now: Utc::now() };
        let id = ApprovalRequestId::generate();
        let issued_by = UserId::new(Uuid::new_v4());
        let request_type = ApprovalRequestType::EditExhibitionInfo {
            description: None,
            icon_key: None,
        };

        let result = ApprovalRequest::create(id, issued_by, request_type, "  ".to_string(), &clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }

    #[test]
    fn test_approve_from_pending() {
        let clock = MockClock { now: Utc::now() };
        let mut request = create_pending_request(&clock);
        let approver = UserId::new(Uuid::new_v4());

        let result = request.approve(approver, Some("Looks good".to_string()), &clock);

        assert!(result.is_ok());
        assert!(matches!(
            request.status(),
            ApprovalRequestStatus::Approved { .. }
        ));
    }

    #[test]
    fn test_approve_from_non_pending_fails() {
        let clock = MockClock { now: Utc::now() };
        let mut request = create_pending_request(&clock);
        let approver = UserId::new(Uuid::new_v4());

        request.approve(approver, None, &clock).unwrap();
        // 2回目の承認は失敗するはず
        let result = request.approve(approver, None, &clock);

        assert!(matches!(result, Err(InvalidTransitionError)));
    }

    #[test]
    fn test_reject_from_pending() {
        let clock = MockClock { now: Utc::now() };
        let mut request = create_pending_request(&clock);
        let rejector = UserId::new(Uuid::new_v4());

        let result = request.reject(rejector, Some("Incomplete info".to_string()), &clock);

        assert!(result.is_ok());
        assert!(matches!(
            request.status(),
            ApprovalRequestStatus::Rejected { .. }
        ));
    }

    #[test]
    fn test_close_from_pending() {
        let clock = MockClock { now: Utc::now() };
        let mut request = create_pending_request(&clock);

        let result = request.close(&clock);

        assert!(result.is_ok());
        assert!(matches!(
            request.status(),
            ApprovalRequestStatus::Closed { .. }
        ));
    }

    fn create_pending_request(clock: &MockClock) -> ApprovalRequest {
        ApprovalRequest::create(
            ApprovalRequestId::generate(),
            UserId::new(Uuid::new_v4()),
            ApprovalRequestType::EditExhibitionInfo {
                description: Some("Test".to_string()),
                icon_key: None,
            },
            "Test reason".to_string(),
            clock,
        )
        .unwrap()
    }
}
