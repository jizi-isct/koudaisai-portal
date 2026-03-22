use std::marker::PhantomData;

use aws_smithy_types::Document;
use uuid::Uuid;

use crate::application::Application;
use crate::application::authz::{self, CanGetByIdError};
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::application::transaction::Transaction;

use crate::application::ports::clock::Clock;

use crate::domain::actor_ctx::{self, ActorContext};

use crate::domain::document_category::DocumentCategory;

pub struct DocumentCategoryApp<'a, Tx: Transaction, DCR: DocumentCategoryRepo<Tx>, C: Clock> {
    _phantom: PhantomData<&'a Tx>,
    document_category_repo: &'a DCR,
    clock: &'a C,
}

impl<'a, Tx: Transaction, DCR: DocumentCategoryRepo<Tx>, C: Clock>
    DocumentCategoryApp<'a, Tx, DCR, C>
{
    pub fn new(document_category_repo: &'a DCR, clock: &'a C) -> Self {
        Self {
            _phantom: PhantomData::default(),
            document_category_repo,
            clock,
        }
    }

    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<DocumentCategory>, ApplicationOperationError<FindError>> {
        // auth
        if !authz::can_get_all_document_categories(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // find all using repo
        Ok(self.document_category_repo.find_all().await?)
    }

    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        id: Uuid,
    ) -> Result<Option<DocumentCategory>, ApplicationOperationError<FindError>> {
        // find user
        let Some(document_category) = self.document_category_repo.find_by_id(id).await? else {
            return Ok(None);
        };

        match authz::can_get_document_category_by_id(actor_ctx) {
            Ok(()) => Ok(Some(document_category)),
            Err(CanGetByIdError::NotFound) => Ok(None),
            Err(CanGetByIdError::Unauthorized) => Err(ApplicationOperationError::Unauthorized),
        }
    }

    pub async fn update_document_category(
        &self,
        actor_ctx: &ActorContext,
        document_category: &DocumentCategory,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        // authz
        if !authz::can_update_document_category(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        Ok(self
            .document_category_repo
            .update(document_category)
            .await?)
    }

    pub async fn delete_document_category(
        &self,
        actor_ctx: &ActorContext,
        id: Uuid,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        // authz
        if !authz::can_delete_document_category(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // delete document_category
        self.document_category_repo.delete(id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::actor_ctx::ActorContext;
    use crate::domain::group::{Group, GroupType};
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::Membership;
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
            claims: vec!["k-portal:admin:all".to_string()],
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
    async fn test_get_all_success_admin() {
        let app = setup_app();
        let ctx = admin_ctx();
        let document_category_app = app.document_category();

        let document_category = DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            &app.clock,
        )
        .unwrap();

        app.document_category_repo
            .insert(document_category)
            .await
            .unwrap();

        let result = document_category_app.get_all(&ctx).await;
        assert!(result.is_ok());
        let document_categories = result.unwrap();
        assert_eq!(document_categories.len(), 1);
        assert_eq!(document_categories[0].id(), document_category.id);
    }

    #[tokio::test]
    async fn test_get_all_success_user() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let group_id = GroupId::new('G', 1).unwrap();
        let membership = Membership::new(group_id, user_id, &app.clock);
        let ctx = user_ctx(user_id, vec![membership.clone()]);
        let document_category_app = app.document_category();

        let document_category = DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            &app.clock,
        )
        .unwrap();

        app.document_category_repo
            .insert(document_category)
            .await
            .unwrap();

        let result = document_category_app.get_all(&ctx).await;
        assert!(result.is_ok());
        let document_categories = result.unwrap();
        assert_eq!(document_categories.len(), 1);
        assert_eq!(document_categories[0].id(), document_category.id);
    }

    #[tokio::test]
    async fn test_get_all_unauthorized() {
        let app = setup_app();
        let ctx = ActorContext::NoLogin;
        let document_category_app = app.document_category();

        let result = document_category_app.get_all(&ctx).await;
        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_get_by_id_success_admin() {
        let app = setup_app();
        let ctx = admin_ctx();
        let document_category_app = app.document_category();

        let document_category = DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            &app.clock,
        )
        .unwrap();

        app.document_category_repo
            .insert(document_category)
            .await
            .unwrap();

        let result = document_category_app
            .get_by_id(&ctx, document_category.id)
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        assert_eq!(result.unwrap().unwrap().id(), document_category.id);
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let app = setup_app();
        let ctx = admin_ctx();
        let document_category_app = app.document_category();
        let result = document_category_app.get_by_id(&ctx, Uuid::new_v4()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let app = setup_app();
        let ctx = ActorContext::NoLogin;
        let document_category_app = app.document_category();
        let result = document_category_app.get_by_id(&ctx, Uuid::new_v4()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_by_id_unauthorized() {
        let app = setup_app();
        let ctx = ActorContext::NoLogin;
        let document_category_app = app.document_category();

        let document_category = DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            &app.clock,
        )
        .unwrap();

        app.document_category_repo
            .insert(document_category)
            .await
            .unwrap();

        let result = document_category_app
            .get_by_id(&ctx, document_category.id)
            .await;
        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_update_document_category_success_admin() {
        let app = setup_app();
        let ctx = admin_ctx();
        let document_category_app = app.document_category();

        let document_category = DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            &app.clock,
        )
        .unwrap();

        let result = document_category_app
            .update_document_category(&ctx, &document_category)
            .await;
        assert!(result.is_ok());

        let stored = app
            .document_category_repo
            .find_by_id(document_category.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(stored.id(), document_category.id);
    }

    #[tokio::test]
    async fn test_update_document_category_unauthorized() {
        let app = setup_app();
        let ctx = ActorContext::NoLogin;
        let document_category_app = app.document_category();

        let document_category = DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            &app.clock,
        )
        .unwrap();

        let result = document_category_app
            .update_document_category(&ctx, &document_category)
            .await;
        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_delete_document_category_success_admin() {
        let app = setup_app();
        let ctx = admin_ctx();
        let document_category_app = app.document_category();

        let document_category = DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            &app.clock,
        )
        .unwrap();

        app.document_category_repo
            .insert(document_category)
            .await
            .unwrap();

        let result = document_category_app
            .delete_document_category(&ctx, document_category.id)
            .await;
        assert!(result.is_ok());

        let stored = app
            .document_category_repo
            .find_by_id(document_category.id)
            .await
            .unwrap();
        assert!(stored.is_none());
    }

    #[tokio::test]
    async fn test_delete_document_category_unauthorized() {
        let app = setup_app();
        let ctx = ActorContext::NoLogin;
        let document_category_app = app.document_category();

        let document_category = DocumentCategory::register(
            Uuid::new_v4(),
            "Test Category".to_string(),
            Some("📃".to_string()),
            &app.clock,
        )
        .unwrap();

        app.document_category_repo
            .insert(document_category)
            .await
            .unwrap();

        let result = document_category_app
            .delete_document_category(&ctx, document_category.id)
            .await;
        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }
}
