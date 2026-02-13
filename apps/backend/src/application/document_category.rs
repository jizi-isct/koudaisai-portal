use std::marker::PhantomData;

use aws_smithy_types::Document;
use uuid::Uuid;

use crate::application::Application;
use crate::application::authz::{self, CanGetByIdError};
use crate::application::error::{ApplicationOperationError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::application::transaction::Transaction;

use crate::application::ports::clock::Clock;

use crate::domain::actor_ctx::{self, ActorContext};

use crate::domain::document_category::DocumentCategory;

pub struct DocumentCategoryApp<'a, Tx: Transaction, DCR: DocumentCategoryRepo<Tx>, C:Clock> {
    _phantom: PhantomData<&'a Tx>,
    document_category_repo: &'a DCR,
    clock: &'a C,
}

impl<'a, Tx: Transaction, DCR: DocumentCategoryRepo<Tx>, C: Clock> DocumentCategoryApp<'a, Tx, DCR, C> {
    pub fn new(document_category_repo: &'a DCR, clock: &'a C) -> Self {
        Self {
            _phantom: PhantomData::default(),
            document_category_repo,
            clock,
        }
    }

    pub async fn get_all(&self, actor_ctx: &ActorContext) -> Result<Vec<DocumentCategory>, ApplicationOperationError<FindError>> {
        // auth
        if !authz::can_get_all_document_categories(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // find all using repo
        Ok(self.document_category_repo.find_all().await?)
    }

    pub async fn get_by_id(&self, actor_ctx: &ActorContext, id: Uuid) -> Result<Option<DocumentCategory>, ApplicationOperationError<FindError>> {
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

    pub async fn update_document_category(&self, actor_ctx: &ActorContext, document_category: &DocumentCategory,) -> Result<(), ApplicationOperationError<UpdateError>> {
        // authz
        if !authz::can_update_document_category(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        Ok(self.document_category_repo.update(document_category).await?)
    }



}



