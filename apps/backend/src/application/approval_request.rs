use std::marker::PhantomData;

use crate::application::authz;
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::application::ports::clock::Clock;
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::transaction::Transaction;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::approval_request::{ApprovalRequest, ApprovalRequestId, ApprovalRequestType};
use crate::domain::user_id::UserId;

pub struct ApprovalRequestApp<
    'a,
    Tx: Transaction,
    AR: ApprovalRequestRepo<Tx>,
    MR: MembershipRepo<Tx>,
    C: Clock,
> {
    _phantom: PhantomData<&'a Tx>,
    approval_request_repo: &'a AR,
    membership_repo: &'a MR,
    clock: &'a C,
}

impl<'a, Tx: Transaction, AR: ApprovalRequestRepo<Tx>, MR: MembershipRepo<Tx>, C: Clock>
    ApprovalRequestApp<'a, Tx, AR, MR, C>
{
    pub fn new(approval_request_repo: &'a AR, membership_repo: &'a MR, clock: &'a C) -> Self {
        Self {
            _phantom: PhantomData,
            approval_request_repo,
            membership_repo,
            clock,
        }
    }

    /// 全ての承認申請を取得（管理者用）
    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<ApprovalRequest>, ApplicationOperationError<FindError>> {
        if !authz::can_get_all_approval_requests(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self.approval_request_repo.find_all().await?)
    }

    /// IDで承認申請を取得
    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
    ) -> Result<Option<ApprovalRequest>, ApplicationOperationError<FindError>> {
        let Some(request) = self.approval_request_repo.find_by_id(id).await? else {
            return Ok(None);
        };

        let memberships_of_issuer = self
            .membership_repo
            .find_by_user_id(request.issued_by())
            .await?;

        if !authz::can_get_approval_request(actor_ctx, &request, &memberships_of_issuer) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        Ok(Some(request))
    }

    /// ユーザーが発行した承認申請を取得
    pub async fn get_by_issued_by(
        &self,
        actor_ctx: &ActorContext,
        user_id: UserId,
    ) -> Result<Vec<ApprovalRequest>, ApplicationOperationError<FindError>> {
        let memberships_of_issuer = self.membership_repo.find_by_user_id(user_id).await?;

        // 同じグループに所属していることを確認するためのダミーリクエストチェック
        // actor_ctxがユーザーの場合、同じグループに属しているかを確認
        match actor_ctx {
            ActorContext::Admin { claims, .. } => {
                if !claims
                    .contains(&"koudaisai-portal:admin:approval-request:read".to_string())
                {
                    return Err(ApplicationOperationError::Unauthorized);
                }
            }
            ActorContext::User { memberships, .. } => {
                let is_same_group = memberships.iter().any(|m| {
                    memberships_of_issuer
                        .iter()
                        .any(|m2| m.group_id() == m2.group_id())
                });
                if !is_same_group {
                    return Err(ApplicationOperationError::Unauthorized);
                }
            }
            ActorContext::NoLogin => {
                return Err(ApplicationOperationError::Unauthorized);
            }
        }

        Ok(self
            .approval_request_repo
            .find_by_issued_by(user_id)
            .await?)
    }

    /// 承認申請を作成
    pub async fn create(
        &self,
        actor_ctx: &ActorContext,
        request_type: ApprovalRequestType,
        issue_reason: String,
    ) -> Result<ApprovalRequestId, ApplicationOperationError<InsertError>> {
        if !authz::can_create_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let user_id = match actor_ctx {
            ActorContext::User { user_id, .. } => *user_id,
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let id = ApprovalRequestId::generate();
        let request =
            ApprovalRequest::create(id, user_id, request_type, issue_reason, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;

        self.approval_request_repo.insert(&request).await?;
        Ok(id)
    }

    /// 承認申請を承認
    pub async fn approve(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
        approval_reason: Option<String>,
    ) -> Result<ApprovalRequest, ApplicationOperationError<UpdateError>> {
        if !authz::can_approve_or_reject_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let approver_id = match actor_ctx {
            ActorContext::Admin { user_id, .. } => *user_id,
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let mut request = self
            .approval_request_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
            .ok_or(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ))?;

        request
            .approve(approver_id, approval_reason, self.clock)
            .map_err(|_| {
                ApplicationOperationError::InvalidInput(
                    "Cannot approve non-pending request".to_string(),
                )
            })?;

        self.approval_request_repo.update(&request).await?;
        Ok(request)
    }

    /// 承認申請を却下
    pub async fn reject(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
        rejection_reason: Option<String>,
    ) -> Result<ApprovalRequest, ApplicationOperationError<UpdateError>> {
        if !authz::can_approve_or_reject_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let rejector_id = match actor_ctx {
            ActorContext::Admin { user_id, .. } => *user_id,
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let mut request = self
            .approval_request_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
            .ok_or(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ))?;

        request
            .reject(rejector_id, rejection_reason, self.clock)
            .map_err(|_| {
                ApplicationOperationError::InvalidInput(
                    "Cannot reject non-pending request".to_string(),
                )
            })?;

        self.approval_request_repo.update(&request).await?;
        Ok(request)
    }

    /// 承認申請をクローズ（申請者によるキャンセル）
    pub async fn close(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        let mut request = self
            .approval_request_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
            .ok_or(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ))?;

        if !authz::can_close_approval_request(actor_ctx, &request) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        request.close(self.clock).map_err(|_| {
            ApplicationOperationError::InvalidInput("Cannot close non-pending request".to_string())
        })?;

        self.approval_request_repo.update(&request).await?;
        Ok(())
    }

    /// 承認申請を削除
    pub async fn delete(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        if !authz::can_delete_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        self.approval_request_repo.delete(id).await?;
        Ok(())
    }
}
