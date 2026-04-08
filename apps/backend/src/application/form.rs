use crate::application::authz;
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::application::ports::clock::Clock;
use crate::application::ports::repositories::form_repo::FormRepo;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::error::FactoryError;
use crate::domain::form::{Form, FormType};
use crate::domain::form_id::FormId;
use crate::domain::target_specifier::TargetSpecifier;
use chrono::{DateTime, Utc};

pub struct FormApp<'a, FR: FormRepo, C: Clock> {
    form_repo: &'a FR,
    clock: &'a C,
}

impl<'a, FR: FormRepo, C: Clock> FormApp<'a, FR, C> {
    pub fn new(form_repo: &'a FR, clock: &'a C) -> Self {
        Self { form_repo, clock }
    }

    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<Form>, ApplicationOperationError<FindError>> {
        Ok(self
            .form_repo
            .find_all()
            .await?
            .into_iter()
            .filter(|f| authz::can_get_form(actor_ctx, f))
            .collect())
    }

    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        form_id: FormId,
    ) -> Result<Option<Form>, ApplicationOperationError<FindError>> {
        let Some(form) = self.form_repo.find_by_id(form_id).await? else {
            return Ok(None);
        };

        if authz::can_get_form(actor_ctx, &form) {
            Ok(Some(form))
        } else {
            Ok(None)
        }
    }

    pub async fn create(
        &self,
        actor_ctx: &ActorContext,
        targets: Vec<TargetSpecifier>,
        name: String,
        summary: String,
        due_date: DateTime<Utc>,
        form_type: FormType,
    ) -> Result<FormId, ApplicationOperationError<InsertError>> {
        if !authz::can_create_form(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let created_by = actor_ctx
            .user_id()
            .ok_or_else(|| ApplicationOperationError::Unauthorized)?;

        let form = match Form::register(
            created_by.into(),
            targets,
            name,
            summary,
            due_date,
            form_type,
            self.clock,
        ) {
            Ok(f) => f,
            Err(FactoryError::InvalidInput(mes)) => {
                return Err(ApplicationOperationError::InvalidInput(mes));
            }
        };

        let form_id = form.id();
        self.form_repo.insert(form).await?;

        Ok(form_id)
    }

    pub async fn update(
        &self,
        actor_ctx: &ActorContext,
        form_id: FormId,
        targets: Option<Vec<TargetSpecifier>>,
        name: Option<String>,
        summary: Option<String>,
        due_date: Option<DateTime<Utc>>,
        form_type: Option<FormType>,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        if !authz::can_update_form(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let Some(mut form) = self
            .form_repo
            .find_by_id(form_id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
        else {
            return Err(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ));
        };

        let updated_by = actor_ctx
            .user_id()
            .ok_or_else(|| ApplicationOperationError::Unauthorized)?;

        if let Some(targets) = targets {
            form.set_targets(targets, Some(updated_by.into()), self.clock);
        }

        if let Some(name) = name {
            form.rename(name, updated_by.into(), self.clock)
                .map_err(|e| match e {
                    FactoryError::InvalidInput(mes) => ApplicationOperationError::InvalidInput(mes),
                })?;
        }

        if let Some(summary) = summary {
            form.change_summary(summary, updated_by.into(), self.clock)
                .map_err(|e| match e {
                    FactoryError::InvalidInput(mes) => ApplicationOperationError::InvalidInput(mes),
                })?;
        }

        if let Some(due_date) = due_date {
            form.change_due_date(due_date, updated_by.into(), self.clock)
                .map_err(|e| match e {
                    FactoryError::InvalidInput(mes) => ApplicationOperationError::InvalidInput(mes),
                })?;
        }

        if let Some(form_type) = form_type {
            form.change_type(form_type, updated_by.into(), self.clock)
                .map_err(|_| {
                    ApplicationOperationError::InvalidInput(
                        "Cannot change form type to a different variant".to_string(),
                    )
                })?;
        }

        self.form_repo.update(form).await?;

        Ok(())
    }

    pub async fn delete(
        &self,
        actor_ctx: &ActorContext,
        form_id: FormId,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        if !authz::can_delete_form(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        self.form_repo.delete(form_id).await?;

        Ok(())
    }
}
