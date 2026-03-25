use std::marker::PhantomData;
use uuid::Uuid;

use crate::application::authz::{self, CanGetByIdError};
use crate::application::error::{ApplicationOperationError, DeleteError, FindError, UpdateError};
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::application::transaction::Transaction;

use crate::application::ports::clock::Clock;

use crate::domain::actor_ctx::ActorContext;

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
        // authz first to avoid leaking resource existence
        match authz::can_get_document_category_by_id(actor_ctx) {
            Ok(()) => {
                // authorized: now query repository
                let document_category = self.document_category_repo.find_by_id(id).await?;
                Ok(document_category)
            }
            Err(CanGetByIdError::NotFound) => Ok(None),
            Err(CanGetByIdError::Unauthorized) => Err(ApplicationOperationError::Unauthorized),
        }
    }

    pub async fn update_document_category(
        &self,
        actor_ctx: &ActorContext,
        id: Uuid,
        title: Option<String>,
        emoji: Option<Option<String>>,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        // authz
        if !authz::can_update_document_category(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let Some(mut document_category) = self
            .document_category_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
        else {
            return Err(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ));
        };

        if let Some(title) = title {
            document_category
                .change_title(title, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        }
        if let Some(emoji) = emoji {
            document_category
                .change_emoji(emoji, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        }

        Ok(self
            .document_category_repo
            .update(&document_category)
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
    use crate::domain::group::GroupType;
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::Membership;
    use crate::domain::user_id::UserId;
    use crate::infra::memory::MemoryApplication;
    use chrono::Utc;
    use uuid::Uuid;

    fn setup_app() -> MemoryApplication {
        MemoryApplication::new_memory_app(Utc::now())
    }

    fn admin_ctx() -> ActorContext {
        ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec![
                "koudaisai-portal:admin:document-category:read".to_string(),
                "koudaisai-portal:admin:document-category:update".to_string(),
                "koudaisai-portal:admin:document-category:delete".to_string(),
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
            .insert(&document_category)
            .await
            .unwrap();

        let result = document_category_app.get_all(&ctx).await;
        assert!(result.is_ok());
        let document_categories = result.unwrap();
        assert_eq!(document_categories.len(), 1);
        assert_eq!(document_categories[0].id(), document_category.id());
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
            .insert(&document_category)
            .await
            .unwrap();

        let result = document_category_app.get_all(&ctx).await;
        assert!(result.is_ok());
        let document_categories = result.unwrap();
        assert_eq!(document_categories.len(), 1);
        assert_eq!(document_categories[0].id(), document_category.id());
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
            .insert(&document_category)
            .await
            .unwrap();

        let result = document_category_app
            .get_by_id(&ctx, document_category.id())
            .await;
        assert!(result.is_ok());

        let document_category_opt = result.unwrap();
        assert!(document_category_opt.is_some());
        assert_eq!(document_category_opt.unwrap().id(), document_category.id());
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
            .insert(&document_category)
            .await
            .unwrap();

        let result = document_category_app
            .get_by_id(&ctx, document_category.id())
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

        app.document_category_repo
            .insert(&document_category)
            .await
            .unwrap();

        let result = document_category_app
            .update_document_category(
                &ctx,
                document_category.id(),
                Some("New Title".to_string()),
                Some(Some("✅".to_string())),
            )
            .await;
        assert!(result.is_ok());

        let stored_after = app
            .document_category_repo
            .find_by_id(document_category.id())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(stored_after.id(), document_category.id());
        assert_eq!(stored_after.title(), "New Title");
        assert_eq!(stored_after.emoji(), Some("✅"));
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
            .update_document_category(
                &ctx,
                document_category.id(),
                Some("New Title".to_string()),
                None,
            )
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
            .insert(&document_category)
            .await
            .unwrap();

        let result = document_category_app
            .delete_document_category(&ctx, document_category.id())
            .await;
        assert!(result.is_ok());

        let stored = app
            .document_category_repo
            .find_by_id(document_category.id())
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
            .insert(&document_category)
            .await
            .unwrap();

        let result = document_category_app
            .delete_document_category(&ctx, document_category.id())
            .await;
        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }
}
