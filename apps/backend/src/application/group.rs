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
        if (!authz::can_get_all_groups(actor_ctx)) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::actor_ctx::ActorContext;
    use crate::domain::group::{Group, GroupType};
    use crate::domain::group_id::GroupId;
    use crate::domain::user_id::UserId;
    use crate::infra::memory::MemoryApplication;
    use crate::infra::memory::transaction_impl::MemoryTransaction;
    use chrono::Utc;
    use uuid::Uuid;

    fn setup_app() -> MemoryApplication {
        MemoryApplication::new_memory_app(Utc::now())
    }

    fn admin_ctx() -> ActorContext {
        ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec![
                "koudaisai-portal:admin:group:read".to_string(),
                "koudaisai-portal:admin:group:create".to_string(),
            ],
        }
    }

    fn user_ctx(user_id: UserId, memberships: Vec<Membership>) -> ActorContext {
        ActorContext::User {
            user_id,
            memberships,
            group_type: GroupType::Press {
                representative: user_id,
            },
        }
    }

    #[tokio::test]
    async fn test_get_all_success() {
        let app = setup_app();
        let ctx = admin_ctx();
        let group_app = app.group();

        let group_id = GroupId::new('G', 1).unwrap();
        let group = Group::register(
            group_id,
            "Test Group".to_string(),
            GroupType::Press {
                representative: UserId::new(Uuid::new_v4()),
            },
            &app.clock,
        )
        .unwrap();
        app.group_repo.insert(group).await.unwrap();

        let result = group_app.get_all(&ctx).await;
        assert!(result.is_ok());
        let groups = result.unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].id(), group_id);
    }

    #[tokio::test]
    async fn test_get_all_unauthorized() {
        let app = setup_app();
        let ctx = ActorContext::NoLogin;
        let group_app = app.group();

        let result = group_app.get_all(&ctx).await;
        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_get_by_id_success_admin() {
        let app = setup_app();
        let ctx = admin_ctx();
        let group_app = app.group();

        let group_id = GroupId::new('G', 1).unwrap();
        let group = Group::register(
            group_id,
            "Test Group".to_string(),
            GroupType::Press {
                representative: UserId::new(Uuid::new_v4()),
            },
            &app.clock,
        )
        .unwrap();
        app.group_repo.insert(group).await.unwrap();

        let result = group_app.get_by_id(&ctx, group_id).await;
        assert!(result.is_ok());
        let group_opt = result.unwrap();
        assert!(group_opt.is_some());
        assert_eq!(group_opt.unwrap().id(), group_id);
    }

    #[tokio::test]
    async fn test_get_by_id_success_user() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let group_id = GroupId::new('G', 1).unwrap();
        let membership = Membership::new(group_id, user_id, &app.clock);
        let ctx = user_ctx(user_id, vec![membership.clone()]);
        let group_app = app.group();

        let group = Group::register(
            group_id,
            "Test Group".to_string(),
            GroupType::Press {
                representative: user_id,
            },
            &app.clock,
        )
        .unwrap();
        app.group_repo.insert(group).await.unwrap();
        app.membership_repo.insert(membership).await.unwrap();

        let result = group_app.get_by_id(&ctx, group_id).await;
        assert!(result.is_ok());
        let group_opt = result.unwrap();
        assert!(group_opt.is_some());
        assert_eq!(group_opt.unwrap().id(), group_id);
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let app = setup_app();
        let ctx = admin_ctx();
        let group_app = app.group();

        let group_id = GroupId::new('G', 1).unwrap();
        let result = group_app.get_by_id(&ctx, group_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_by_id_unauthorized_user() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = user_ctx(user_id, vec![]); // No memberships
        let group_app = app.group();

        let group_id = GroupId::new('G', 1).unwrap();
        let group = Group::register(
            group_id,
            "Test Group".to_string(),
            GroupType::Press {
                representative: UserId::new(Uuid::new_v4()),
            },
            &app.clock,
        )
        .unwrap();
        app.group_repo.insert(group).await.unwrap();

        let result = group_app.get_by_id(&ctx, group_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // authz returns NotFound for users not in group
    }

    #[tokio::test]
    async fn test_create_group_success() {
        let app = setup_app();
        let ctx = admin_ctx();
        let group_app = app.group();
        let tx = MemoryTransaction::new();

        let group_id = GroupId::new('G', 1).unwrap();
        let rep_id = UserId::new(Uuid::new_v4());
        let group_type = GroupType::Press {
            representative: rep_id,
        };

        let result = group_app
            .create_group(&ctx, tx, group_id, "New Group".to_string(), group_type)
            .await;
        assert!(result.is_ok());

        let saved_group = app.group_repo.find_by_id(group_id).await.unwrap();
        assert!(saved_group.is_some());
        assert_eq!(saved_group.unwrap().name(), "New Group");

        let memberships = app
            .membership_repo
            .find_by_group_id(group_id)
            .await
            .unwrap();
        assert_eq!(memberships.len(), 1);
        assert_eq!(memberships[0].user_id(), rep_id);
    }

    #[tokio::test]
    async fn test_create_group_unauthorized() {
        let app = setup_app();
        let ctx = ActorContext::NoLogin;
        let group_app = app.group();
        let tx = MemoryTransaction::new();

        let group_id = GroupId::new('G', 1).unwrap();
        let result = group_app
            .create_group(
                &ctx,
                tx,
                group_id,
                "New Group".to_string(),
                GroupType::Press {
                    representative: UserId::new(Uuid::new_v4()),
                },
            )
            .await;
        assert!(matches!(
            result,
            Err(ApplicationSequentialOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_create_group_invalid_input() {
        let app = setup_app();
        let ctx = admin_ctx();
        let group_app = app.group();
        let tx = MemoryTransaction::new();

        let group_id = GroupId::new('G', 1).unwrap();
        let result = group_app
            .create_group(
                &ctx,
                tx,
                group_id,
                "".to_string(),
                GroupType::Press {
                    representative: UserId::new(Uuid::new_v4()),
                },
            )
            .await;
        assert!(matches!(
            result,
            Err(ApplicationSequentialOperationError::InvalidInput(_))
        ));
    }
}
