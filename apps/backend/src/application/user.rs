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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::actor_ctx::ActorContext;
    use crate::domain::email_address::EmailAddress;
    use crate::domain::user::User;
    use crate::domain::user_id::UserId;
    use crate::infra::memory::MemoryApplication;
    use chrono::Utc;
    use uuid::Uuid;

    async fn setup() -> (MemoryApplication, UserId) {
        let now = Utc::now();
        let app = MemoryApplication::new_memory_app(now);
        let user_id = UserId::new(Uuid::new_v4());
        let email = EmailAddress::new("test@example.com".to_string()).unwrap();
        let user =
            User::register(user_id, "Test User".to_string(), email, app.clock.clone()).unwrap();
        app.user_repo.insert(&user).await.unwrap();
        (app, user_id)
    }

    #[tokio::test]
    async fn test_get_all_success() {
        let (app, _) = setup().await;
        let actor_ctx = ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec!["koudaisai-portal:admin:user:read".to_string()],
        };

        let user_app = app.user();
        let result = user_app.get_all(&actor_ctx).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_all_unauthorized() {
        let (app, _) = setup().await;
        let actor_ctx = ActorContext::NoLogin;

        let user_app = app.user();
        let result = user_app.get_all(&actor_ctx).await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_get_by_id_success_admin() {
        let (app, user_id) = setup().await;
        let actor_ctx = ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec!["koudaisai-portal:admin:user:read".to_string()],
        };

        let user_app = app.user();
        let result = user_app.get_by_id(&actor_ctx, user_id).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert!(user.is_some());
        assert_eq!(user.unwrap().id(), user_id);
    }

    #[tokio::test]
    async fn test_get_by_id_success_user() {
        let (app, user_id) = setup().await;
        // Mock group_id
        use crate::domain::group_id::GroupId;
        let group_id = GroupId::new('G', 1).unwrap();

        // Register a membership for the target user
        use crate::domain::membership::Membership;
        let membership = Membership::new(group_id, user_id, &app.clock);
        app.membership_repo
            .insert(membership.clone())
            .await
            .unwrap();

        // Actor is in the same group
        let actor_id = UserId::new(Uuid::new_v4());
        let actor_membership = Membership::new(group_id, actor_id, &app.clock);
        let actor_ctx = ActorContext::User {
            user_id: actor_id,
            memberships: vec![actor_membership],
            group_type: crate::domain::group::GroupType::Press {
                representative: actor_id,
            },
        };

        let user_app = app.user();
        let result = user_app.get_by_id(&actor_ctx, user_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let (app, _) = setup().await;
        let actor_ctx = ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec!["koudaisai-portal:admin:user:read".to_string()],
        };

        let user_app = app.user();
        let result = user_app
            .get_by_id(&actor_ctx, UserId::new(Uuid::new_v4()))
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_by_id_unauthorized() {
        let (app, user_id) = setup().await;
        // Actor is another user not in the same group
        let actor_ctx = ActorContext::User {
            user_id: UserId::new(Uuid::new_v4()),
            memberships: vec![],
            group_type: crate::domain::group::GroupType::Press {
                representative: UserId::new(Uuid::new_v4()),
            },
        };

        let user_app = app.user();
        let result = user_app.get_by_id(&actor_ctx, user_id).await;

        // authz::can_get_user_by_id returns Err(CanGetByIdError::NotFound) if not in same group
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_user_success() {
        let (app, user_id) = setup().await;
        let actor_ctx = ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec!["koudaisai-portal:admin:user:update".to_string()],
        };

        let mut user = app.user_repo.find_by_id(user_id).await.unwrap().unwrap();
        user.rename("Updated Name".to_string(), app.clock.clone())
            .unwrap();

        let user_app = app.user();
        let result = user_app.update_user(&actor_ctx, &user).await;

        assert!(result.is_ok());

        let updated_user = app.user_repo.find_by_id(user_id).await.unwrap().unwrap();
        assert_eq!(updated_user.name(), "Updated Name");
    }

    #[tokio::test]
    async fn test_update_user_unauthorized() {
        let (app, user_id) = setup().await;
        let actor_ctx = ActorContext::NoLogin;

        let user = app.user_repo.find_by_id(user_id).await.unwrap().unwrap();

        let user_app = app.user();
        let result = user_app.update_user(&actor_ctx, &user).await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_change_m_address_success() {
        let (app, user_id) = setup().await;
        let actor_ctx = ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec!["koudaisai-portal:admin:user:change-email".to_string()],
        };
        let new_email = EmailAddress::new("new@example.com".to_string()).unwrap();

        let user_app = app.user();
        let result = user_app
            .change_m_address(&actor_ctx, user_id, new_email.clone())
            .await;

        assert!(result.is_ok());

        let updated_user = app.user_repo.find_by_id(user_id).await.unwrap().unwrap();
        assert_eq!(updated_user.m_address(), &new_email);
    }

    #[tokio::test]
    async fn test_change_m_address_unauthorized() {
        let (app, user_id) = setup().await;
        let actor_ctx = ActorContext::NoLogin;
        let new_email = EmailAddress::new("new@example.com".to_string()).unwrap();

        let user_app = app.user();
        let result = user_app
            .change_m_address(&actor_ctx, user_id, new_email)
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_change_m_address_not_found() {
        let (app, _) = setup().await;
        let actor_ctx = ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec!["koudaisai-portal:admin:user:change-email".to_string()],
        };
        let new_email = EmailAddress::new("new@example.com".to_string()).unwrap();

        let user_app = app.user();
        let result = user_app
            .change_m_address(&actor_ctx, UserId::new(Uuid::new_v4()), new_email)
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound
            ))
        ));
    }
}
