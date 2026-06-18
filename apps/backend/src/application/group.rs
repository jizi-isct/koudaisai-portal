use crate::application::authz;
use crate::application::authz::CanGetByIdError;
use crate::application::error::{
    ApplicationOperationError, ApplicationSequentialOperationError, DeleteError, FindError,
    InsertError,
};
use crate::application::ports::clock::Clock;
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::transaction::Transaction;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::error::FactoryError;
use crate::domain::group::{Group, GroupType};
use crate::domain::group_id::GroupId;
use crate::domain::membership::{Membership, Role};
use crate::domain::user_id::UserId;
use std::marker::PhantomData;

pub struct GroupApp<
    'a,
    Tx: Transaction,
    GR: GroupRepo<Tx>,
    MR: MembershipRepo<Tx>,
    UR: UserRepo<Tx>,
    C: Clock,
> {
    _phantom: std::marker::PhantomData<&'a Tx>,
    group_repo: &'a GR,
    membership_repo: &'a MR,
    user_repo: &'a UR,
    clock: &'a C,
}

impl<'a, Tx: Transaction, GR: GroupRepo<Tx>, MR: MembershipRepo<Tx>, UR: UserRepo<Tx>, C: Clock>
    GroupApp<'a, Tx, GR, MR, UR, C>
{
    pub fn new(
        group_repo: &'a GR,
        membership_repo: &'a MR,
        user_repo: &'a UR,
        clock: &'a C,
    ) -> Self {
        Self {
            _phantom: PhantomData::default(),
            group_repo,
            membership_repo,
            user_repo,
            clock,
        }
    }

    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<Group>, ApplicationOperationError<FindError>> {
        // auth
        if !authz::can_get_all_groups(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // find all using repo
        Ok(self.group_repo.find_all().await?)
    }

    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        group_id: GroupId,
    ) -> Result<Option<Group>, ApplicationOperationError<FindError>> {
        // find group
        let Some(group) = self.group_repo.find_by_id(group_id).await? else {
            return Ok(None);
        };

        // get membership
        let members = self.membership_repo.find_by_group_id(group_id).await?;

        // auth and return
        match authz::can_get_group_by_id(actor_ctx, &members) {
            Ok(()) => Ok(Some(group)),
            Err(CanGetByIdError::NotFound) => Ok(None),
            Err(CanGetByIdError::Unauthorized) => Err(ApplicationOperationError::Unauthorized),
        }
    }

    pub async fn create_group(
        &self,
        actor_ctx: &ActorContext,
        mut tx: Tx,
        group_id: GroupId,
        group_name: String,
        group_type: GroupType,
    ) -> Result<(), ApplicationSequentialOperationError<InsertError>> {
        // auth
        if !authz::can_create_group(actor_ctx) {
            return Err(ApplicationSequentialOperationError::Unauthorized);
        }

        // create a group (membership is managed independently)
        let group = match Group::register(group_id, group_name, group_type, self.clock) {
            Ok(g) => g,
            Err(FactoryError::InvalidInput(mes)) => {
                return Err(ApplicationSequentialOperationError::InvalidInput(mes));
            }
        };

        // save group
        tx.begin().await?;
        self.group_repo.insert_in(&mut tx, group).await?;
        tx.commit().await?;

        Ok(())
    }

    /// グループに役職付きでメンバーを追加します．
    /// 追加後のメンバー集合がグループ種別に対して構造的に妥当か（許可役職・各役職 `<= 1`・責任者相異）を
    /// 検証してから保存します．役職変更は「対象役職を remove → 新メンバーを add」で表現します．
    pub async fn add_member(
        &self,
        actor_ctx: &ActorContext,
        mut tx: Tx,
        group_id: GroupId,
        user_id: UserId,
        role: Role,
    ) -> Result<(), ApplicationSequentialOperationError<InsertError>> {
        // auth
        if !authz::can_manage_group_members(actor_ctx) {
            return Err(ApplicationSequentialOperationError::Unauthorized);
        }

        // グループの存在確認と種別の取得
        let group = self
            .group_repo
            .find_by_id(group_id)
            .await
            .map_err(|e| ApplicationSequentialOperationError::InternalError(e.into()))?;
        let Some(group) = group else {
            return Err(ApplicationSequentialOperationError::InvalidInput(
                "group not found".to_string(),
            ));
        };

        // 追加対象ユーザーの存在確認
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| ApplicationSequentialOperationError::InternalError(e.into()))?;
        if user.is_none() {
            return Err(ApplicationSequentialOperationError::InvalidInput(
                "user not found".to_string(),
            ));
        }

        // 追加後の集合を組み立てて構造妥当性を検証
        let membership = Membership::new(group_id, user_id, role, self.clock);
        let mut members = self
            .membership_repo
            .find_by_group_id(group_id)
            .await
            .map_err(|e| ApplicationSequentialOperationError::InternalError(e.into()))?;
        members.push(membership.clone());
        if let Err(FactoryError::InvalidInput(mes)) =
            Membership::validate_set(*group.r#type(), &members)
        {
            return Err(ApplicationSequentialOperationError::InvalidInput(mes));
        }

        // 保存（役職スロットの重複は repo の (group_id, role) 一意制約でも弾かれる）
        tx.begin().await?;
        self.membership_repo.insert_in(&mut tx, membership).await?;
        tx.commit().await?;

        Ok(())
    }

    /// グループから指定された役職スロットのメンバーを削除します（空席化）．
    /// 削除は構造妥当性を緩める方向のみなので追加検証は不要です．
    pub async fn remove_member(
        &self,
        actor_ctx: &ActorContext,
        mut tx: Tx,
        group_id: GroupId,
        role: Role,
    ) -> Result<(), ApplicationSequentialOperationError<DeleteError>> {
        // auth
        if !authz::can_manage_group_members(actor_ctx) {
            return Err(ApplicationSequentialOperationError::Unauthorized);
        }

        // 対象の役職スロットに居るメンバーを特定
        let members = self
            .membership_repo
            .find_by_group_id(group_id)
            .await
            .map_err(|e| ApplicationSequentialOperationError::InternalError(e.into()))?;
        let Some(membership) = members.into_iter().find(|m| m.role() == role) else {
            return Err(ApplicationSequentialOperationError::OperationFailed(
                DeleteError::NotFound,
            ));
        };

        // 削除
        tx.begin().await?;
        self.membership_repo.delete_in(&mut tx, membership).await?;
        tx.commit().await?;

        Ok(())
    }
}
