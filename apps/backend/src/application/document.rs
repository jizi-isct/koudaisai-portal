use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

use crate::application::authz::{self, CanGetByIdError, can_get_document};
use crate::application::transaction::Transaction;

use crate::application::ports::clock::Clock;

use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::application::ports::repositories::document_repo::DocumentRepo;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::admin_id::AdminId;
use crate::domain::document::Document;
use crate::domain::document::DocumentFormat;
use crate::domain::group::GroupType;
use crate::domain::membership::Membership;
use crate::domain::target_specifier::TargetSpecifier;
use crate::domain::user_id::UserId;

pub struct DocumentApp<'a, Tx: Transaction, DR: DocumentRepo<Tx>, C: Clock> {
    _phantom: PhantomData<&'a Tx>,
    document_repo: &'a DR,
    clock: &'a C,
}

impl<'a, Tx: Transaction, DR: DocumentRepo<Tx>, C: Clock> DocumentApp<'a, Tx, DR, C> {
    pub fn new(document_repo: &'a DR, clock: &'a C) -> Self {
        Self {
            _phantom: PhantomData,
            document_repo,
            clock,
        }
    }

    // 文書可視性フィルタ用のターゲット展開ヘルパー。ハンドラ未接続だが
    // 可視性絞り込みの再配線で利用予定のため保持する。
    #[allow(dead_code)]
    fn get_target_specifier(
        user_id: &UserId,
        memberships: &[Membership],
        group_type: &GroupType,
    ) -> Vec<TargetSpecifier> {
        let mut targets = vec![
            TargetSpecifier::UserNologin,
            TargetSpecifier::UserId(*user_id),
        ];
        for membership in memberships {
            targets.push(TargetSpecifier::GroupId(membership.group_id()));
        }
        targets.push(match group_type {
            crate::domain::group::GroupType::GeneralProject => {
                TargetSpecifier::GroupTypeProjectGeneral
            }
            crate::domain::group::GroupType::BoothProject => TargetSpecifier::GroupTypeProjectBooth,
            crate::domain::group::GroupType::StageProject => TargetSpecifier::GroupTypeProjectStage,
            crate::domain::group::GroupType::LabProject => TargetSpecifier::GroupTypeProjectLabo,
            crate::domain::group::GroupType::Press => TargetSpecifier::GroupTypePress,
        });

        targets
    }

    pub async fn create(
        &self,
        actor_ctx: &ActorContext,
        title: String,
        category: Option<Uuid>,
        targets: Vec<TargetSpecifier>,
        format: DocumentFormat,
    ) -> Result<Document, ApplicationOperationError<InsertError>> {
        if !authz::can_create_document(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let created_by = AdminId::new(
            actor_ctx
                .user_id()
                .ok_or_else(|| ApplicationOperationError::Unauthorized)?
                .into(),
        );

        let document = Document::register(title, category, format, targets, created_by, self.clock)
            .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;

        self.document_repo.insert(&document).await?;
        Ok(document)
    }

    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<Document>, ApplicationOperationError<FindError>> {
        Ok(self
            .document_repo
            .find_all()
            .await?
            .into_iter()
            .filter(|doc| can_get_document(actor_ctx, doc).is_ok())
            .collect())
    }

    pub async fn get_by_category(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<HashMap<Option<Uuid>, Vec<Document>>, ApplicationOperationError<FindError>> {
        let documents = self
            .document_repo
            .find_all()
            .await?
            .into_iter()
            .filter(|doc| can_get_document(actor_ctx, doc).is_ok())
            .fold(HashMap::new(), |mut acc, doc| {
                acc.entry(doc.category()).or_insert_with(Vec::new).push(doc);
                acc
            });
        Ok(documents)
    }

    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        id: Uuid,
    ) -> Result<Option<Document>, ApplicationOperationError<FindError>> {
        let Some(document) = self.document_repo.find_by_id(id).await? else {
            return Ok(None);
        };

        match authz::can_get_document(actor_ctx, &document) {
            Ok(()) => Ok(Some(document)),
            Err(CanGetByIdError::NotFound) => Ok(None),
            Err(CanGetByIdError::Unauthorized) => Err(ApplicationOperationError::Unauthorized),
        }
    }

    pub async fn update_document(
        &self,
        actor_ctx: &ActorContext,
        id: Uuid,
        title: Option<String>,
        category: Option<Option<Uuid>>,
        format: Option<DocumentFormat>,
        targets: Option<Vec<TargetSpecifier>>,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        if !authz::can_update_document(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let Some(mut document) = self.document_repo.find_by_id(id).await.map_err(|e| {
            ApplicationOperationError::OperationFailed(UpdateError::InternalError(e.into()))
        })?
        else {
            return Err(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ));
        };

        let updated_by = AdminId::new(
            actor_ctx
                .user_id()
                .ok_or_else(|| {
                    ApplicationOperationError::InternalError(anyhow::anyhow!("missing user_id"))
                })?
                .into(),
        );

        if let Some(title) = title {
            document
                .change_title(title, updated_by, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        }
        if let Some(category) = category {
            document
                .change_category(category, updated_by, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        }
        if let Some(format) = format {
            document
                .change_format(format, updated_by, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        }
        if let Some(targets) = targets {
            document
                .change_targets(targets, updated_by, self.clock)
                .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;
        }

        Ok(self.document_repo.update(&document).await?)
    }

    pub async fn delete_document(
        &self,
        actor_ctx: &ActorContext,
        id: Uuid,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        // authz
        if !authz::can_delete_document(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // delete document
        self.document_repo.delete(id).await?;
        Ok(())
    }
}
