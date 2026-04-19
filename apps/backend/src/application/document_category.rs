use std::marker::PhantomData;
use uuid::Uuid;

use crate::application::authz::{self, CanGetByIdError};
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
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

    pub async fn create(
        &self,
        actor_ctx: &ActorContext,
        title: String,
        emoji: Option<String>,
    ) -> Result<DocumentCategory, ApplicationOperationError<InsertError>> {
        if !authz::can_create_document_category(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        let category = DocumentCategory::register(title, emoji, self.clock)
            .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        self.document_category_repo.insert(&category).await?;
        Ok(category)
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
            .map_err(|e| {
                ApplicationOperationError::OperationFailed(UpdateError::InternalError(e.into()))
            })?
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
