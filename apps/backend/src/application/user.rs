use crate::application::authz;
use crate::application::authz::CanGetByIdError;
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::domain::error::FactoryError;
use crate::application::ports::clock::Clock;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::transaction::Transaction;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::email_address::EmailAddress;
use crate::domain::group_id::GroupId;
use crate::domain::user::User;
use crate::domain::user_id::UserId;
use std::marker::PhantomData;

pub struct UserApp<'a, Tx: Transaction, MR: MembershipRepo<Tx>, UR: UserRepo<Tx>, C: Clock> {
    _phantom: PhantomData<&'a Tx>,
    membership_repo: &'a MR,
    user_repo: &'a UR,
    clock: &'a C,
}

impl<'a, Tx: Transaction, MR: MembershipRepo<Tx>, UR: UserRepo<Tx>, C: Clock>
    UserApp<'a, Tx, MR, UR, C>
{
    pub fn new(membership_repo: &'a MR, user_repo: &'a UR, clock: &'a C) -> Self {
        Self {
            _phantom: PhantomData::default(),
            membership_repo,
            user_repo,
            clock,
        }
    }

    pub async fn register(
        &self,
        actor_ctx: &ActorContext,
        user_id: UserId,
        name: String,
        m_address: EmailAddress,
    ) -> Result<User, ApplicationOperationError<InsertError>>
    where
        &'a C: Clock,
    {
        if !authz::can_get_all_users(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        let user = User::register(user_id, name, m_address, self.clock)
            .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        self.user_repo.insert(&user).await?;
        Ok(user)
    }

    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<User>, ApplicationOperationError<FindError>> {
        // auth
        if !authz::can_get_all_users(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // find all using repo
        Ok(self.user_repo.find_all().await?)
    }

    /// ユーザーと代表グループ ID(最初の所属)を返す。`group_id` は表示や
    /// プラン情報参照のために使う(`build_actor_context` と同じ「最初の所属」慣習)。
    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        id: UserId,
    ) -> Result<Option<(User, Option<GroupId>)>, ApplicationOperationError<FindError>> {
        // find user
        let Some(user) = self.user_repo.find_by_id(id).await? else {
            return Ok(None);
        };

        // get membership
        let members = self.membership_repo.find_by_user_id(id).await?;
        let primary_group = members.first().map(|m| m.group_id());

        // auth and return
        match authz::can_get_user_by_id(actor_ctx, members) {
            Ok(()) => Ok(Some((user, primary_group))),
            Err(CanGetByIdError::NotFound) => Ok(None),
            Err(CanGetByIdError::Unauthorized) => Err(ApplicationOperationError::Unauthorized),
        }
    }

    pub async fn update_user(
        &self,
        actor_ctx: &ActorContext,
        user: &User,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        // authz
        if !authz::can_update_user(actor_ctx, user) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // update user
        Ok(self.user_repo.update(user).await?)
    }

    pub async fn change_m_address(
        &self,
        actor_ctx: &ActorContext,
        user_id: UserId,
        new_address: EmailAddress,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        // authz
        if !authz::can_change_m_address_of_the_user(actor_ctx, user_id) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // get user
        let user = self.user_repo.find_by_id(user_id).await;
        let mut user = match user {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(ApplicationOperationError::OperationFailed(
                    UpdateError::NotFound,
                ));
            }
            Err(FindError::InternalError(e)) => {
                return Err(ApplicationOperationError::InternalError(e));
            }
        };

        // update user
        user.change_m_address(new_address, self.clock);

        // save user
        self.user_repo.update(&user).await?;
        Ok(())
    }

    /// ユーザーの氏名・m アドレスを部分更新します（`PATCH /users/{id}`）。
    /// activation トークンの再発行は伴いません（再発行は専用エンドポイント）。
    pub async fn edit_user(
        &self,
        actor_ctx: &ActorContext,
        user_id: UserId,
        name: Option<String>,
        m_address: Option<EmailAddress>,
    ) -> Result<User, ApplicationOperationError<UpdateError>> {
        // get user
        let Some(mut user) = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
        else {
            return Err(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ));
        };

        // authz（管理者の user:update が必要）
        if !authz::can_update_user(actor_ctx, &user) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        if let Some(name) = name {
            user.rename(name, self.clock)
                .map_err(|FactoryError::InvalidInput(mes)| {
                    ApplicationOperationError::InvalidInput(mes)
                })?;
        }
        if let Some(m_address) = m_address {
            user.change_m_address(m_address, self.clock);
        }

        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// ユーザーを削除します。
    pub async fn delete_user(
        &self,
        actor_ctx: &ActorContext,
        user_id: UserId,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        // authz
        if !authz::can_delete_user(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        self.user_repo.delete(user_id).await?;
        Ok(())
    }
}
