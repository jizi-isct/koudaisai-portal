use std::marker::PhantomData;
use crate::application::authz;
use crate::application::authz::CanGetByIdError;
use crate::application::error::{ApplicationOperationError, FindError, InsertError, UpdateError};
use crate::application::ports::clock::Clock;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::transaction::Transaction;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::email_address::EmailAddress;
use crate::domain::user::User;
use crate::domain::user_id::UserId;

pub struct UserApp<'a, Tx: Transaction, MR: MembershipRepo<Tx>, UR: UserRepo<Tx>, C: Clock> {
    _phantom: PhantomData<&'a Tx>,
    membership_repo: &'a MR,
    user_repo: &'a UR,
    clock: &'a C,
}

impl<'a, Tx: Transaction, MR: MembershipRepo<Tx>, UR: UserRepo<Tx>, C: Clock> UserApp<'a, Tx, MR, UR, C> {
    pub fn new(membership_repo: &'a MR, user_repo: &'a UR, clock: &'a C) -> Self {
        Self {
            _phantom: PhantomData::default(),
            membership_repo,
            user_repo,
            clock,
        }
    }

    pub async fn get_all(&self, actor_ctx: &ActorContext) -> Result<Vec<User>, ApplicationOperationError<FindError>> {
        // auth
        if (!authz::can_get_all_users(actor_ctx)) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // find all using repo
        Ok(self.user_repo.find_all().await?)
    }

    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        id: UserId,
    ) -> Result<Option<User>, ApplicationOperationError<FindError>> {
        // find user
        let Some(user) = self.user_repo.find_by_id(id).await? else {
            return Ok(None);
        };

        // get membership
        let members = self.membership_repo.find_by_user_id(id).await?;

        // auth and return
        match authz::can_get_user_by_id(actor_ctx, members) {
            Ok(()) => Ok(Some(user)),
            Err(CanGetByIdError::NotFound) => Ok(None),
            Err(CanGetByIdError::Unauthorized) => Err(ApplicationOperationError::Unauthorized),
        }
    }

    pub async fn update_user(
        &self,
        actor_ctx: &ActorContext,
        user: &User,
    ) -> Result<(), ApplicationOperationError<InsertError>> {
        // authz
        if !authz::can_update_user(actor_ctx, user) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // update user
        Ok(self.user_repo.insert(user).await?)
    }

    pub async fn change_m_address(
        &self,
        actor_ctx: &ActorContext,
        user_id: UserId,
        new_address: EmailAddress,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        // authz
        if (!authz::can_change_m_address_of_the_user(actor_ctx, user_id)) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // get user
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await;
        let mut user = match user {
            Ok(Some(user)) => user,
            Ok(None) => return Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)),
            Err(FindError::InternalError(e)) => return Err(ApplicationOperationError::InternalError(e)),
        };

        // update user
        user.change_m_address(new_address, self.clock);

        // save user
        self.user_repo.update(&user).await?;
        Ok(())
    }
}
