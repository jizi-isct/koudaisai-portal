use crate::application::authz;
use crate::application::authz::CanGetByIdError;
use crate::application::error::{
    ApplicationOperationError, ApplicationSequentialOperationError, FindError, InsertError,
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
use crate::domain::membership::Membership;
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

        // create a group
        let group = match Group::register(group_id, group_name, group_type, self.clock) {
            Ok(g) => g,
            Err(FactoryError::InvalidInput(mes)) => {
                return Err(ApplicationSequentialOperationError::InvalidInput(mes));
            }
        };

        // create memberships
        let memberships = Membership::from_group_type(group_id, group.r#type(), self.clock);

        // save groups and memberships
        tx.begin().await?;
        self.group_repo.insert_in(&mut tx, group).await?;
        for membership in memberships {
            self.membership_repo.insert_in(&mut tx, membership).await?;
        }
        tx.commit().await?;

        Ok(())
    }
}
