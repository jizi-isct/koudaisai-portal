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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::actor_ctx::ActorContext;
    use crate::domain::form::{Form, FormType};
    use crate::domain::group::GroupType;
    use crate::domain::membership::Membership;
    use crate::domain::target_specifier::TargetSpecifier;
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
                "koudaisai-portal:admin:form:read".to_string(),
                "koudaisai-portal:admin:form:create".to_string(),
                "koudaisai-portal:admin:form:update".to_string(),
                "koudaisai-portal:admin:form:delete".to_string(),
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

    async fn create_test_form(app: &MemoryApplication, targets: Vec<TargetSpecifier>) -> Form {
        let form = Form::register(
            Uuid::new_v4(),
            targets,
            "Test Form".to_string(),
            "Test Summary".to_string(),
            Utc::now() + chrono::Duration::days(7),
            FormType::TypeExternal {
                form_url: "https://example.com/form".to_string(),
            },
            &app.clock,
        )
        .unwrap();
        app.form_repo.insert(form.clone()).await.unwrap();
        form
    }

    #[tokio::test]
    async fn test_get_all_admin() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app.get_all(&ctx).await;
        assert!(result.is_ok());
        let forms = result.unwrap();
        assert_eq!(forms.len(), 1);
        assert_eq!(forms[0].id(), form.id());
    }

    #[tokio::test]
    async fn test_get_all_user_with_matching_target() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = user_ctx(user_id, vec![]);
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app.get_all(&ctx).await;
        assert!(result.is_ok());
        let forms = result.unwrap();
        assert_eq!(forms.len(), 1);
        assert_eq!(forms[0].id(), form.id());
    }

    #[tokio::test]
    async fn test_get_all_user_without_matching_target() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = user_ctx(user_id, vec![]);
        let form_app = app.form();

        create_test_form(&app, vec![TargetSpecifier::GroupTypeProjectGeneral]).await;

        let result = form_app.get_all(&ctx).await;
        assert!(result.is_ok());
        let forms = result.unwrap();
        assert_eq!(forms.len(), 0);
    }

    #[tokio::test]
    async fn test_get_by_id_admin() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app.get_by_id(&ctx, form.id()).await;
        assert!(result.is_ok());
        let form_opt = result.unwrap();
        assert!(form_opt.is_some());
        assert_eq!(form_opt.unwrap().id(), form.id());
    }

    #[tokio::test]
    async fn test_get_by_id_user_with_matching_target() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = user_ctx(user_id, vec![]);
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app.get_by_id(&ctx, form.id()).await;
        assert!(result.is_ok());
        let form_opt = result.unwrap();
        assert!(form_opt.is_some());
        assert_eq!(form_opt.unwrap().id(), form.id());
    }

    #[tokio::test]
    async fn test_get_by_id_user_without_matching_target() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = user_ctx(user_id, vec![]);
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypeProjectGeneral]).await;

        let result = form_app.get_by_id(&ctx, form.id()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let form_id = FormId::new(Uuid::new_v4());
        let result = form_app.get_by_id(&ctx, form_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_create_success() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let result = form_app
            .create(
                &ctx,
                vec![TargetSpecifier::GroupTypePress],
                "New Form".to_string(),
                "New Summary".to_string(),
                Utc::now() + chrono::Duration::days(7),
                FormType::TypeExternal {
                    form_url: "https://example.com/new".to_string(),
                },
            )
            .await;

        assert!(result.is_ok());
        let form_id = result.unwrap();

        let saved_form = app.form_repo.find_by_id(form_id).await.unwrap();
        assert!(saved_form.is_some());
        assert_eq!(saved_form.unwrap().name(), "New Form");
    }

    #[tokio::test]
    async fn test_create_unauthorized() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = user_ctx(user_id, vec![]);
        let form_app = app.form();

        let result = form_app
            .create(
                &ctx,
                vec![TargetSpecifier::GroupTypePress],
                "New Form".to_string(),
                "New Summary".to_string(),
                Utc::now() + chrono::Duration::days(7),
                FormType::TypeExternal {
                    form_url: "https://example.com/new".to_string(),
                },
            )
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_create_invalid_input() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let result = form_app
            .create(
                &ctx,
                vec![TargetSpecifier::GroupTypePress],
                "".to_string(),
                "New Summary".to_string(),
                Utc::now() + chrono::Duration::days(7),
                FormType::TypeExternal {
                    form_url: "https://example.com/new".to_string(),
                },
            )
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::InvalidInput(_))
        ));
    }

    #[tokio::test]
    async fn test_update_success() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app
            .update(
                &ctx,
                form.id(),
                None,
                Some("Updated Form".to_string()),
                Some("Updated Summary".to_string()),
                None,
                None,
            )
            .await;

        assert!(result.is_ok());

        let updated_form = app.form_repo.find_by_id(form.id()).await.unwrap().unwrap();
        assert_eq!(updated_form.name(), "Updated Form");
        assert_eq!(updated_form.summary(), "Updated Summary");
    }

    #[tokio::test]
    async fn test_update_unauthorized() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = user_ctx(user_id, vec![]);
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app
            .update(
                &ctx,
                form.id(),
                None,
                Some("Updated Form".to_string()),
                None,
                None,
                None,
            )
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let form_id = FormId::new(Uuid::new_v4());
        let result = form_app
            .update(
                &ctx,
                form_id,
                None,
                Some("Updated Form".to_string()),
                None,
                None,
                None,
            )
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound
            ))
        ));
    }

    #[tokio::test]
    async fn test_update_invalid_input() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app
            .update(
                &ctx,
                form.id(),
                None,
                Some("".to_string()),
                None,
                None,
                None,
            )
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::InvalidInput(_))
        ));
    }

    #[tokio::test]
    async fn test_delete_success() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app.delete(&ctx, form.id()).await;
        assert!(result.is_ok());

        let deleted_form = app.form_repo.find_by_id(form.id()).await.unwrap();
        assert!(deleted_form.is_none());
    }

    #[tokio::test]
    async fn test_delete_unauthorized() {
        let app = setup_app();
        let user_id = UserId::new(Uuid::new_v4());
        let ctx = user_ctx(user_id, vec![]);
        let form_app = app.form();

        let form = create_test_form(&app, vec![TargetSpecifier::GroupTypePress]).await;

        let result = form_app.delete(&ctx, form.id()).await;
        assert!(matches!(
            result,
            Err(ApplicationOperationError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn test_delete_not_found() {
        let app = setup_app();
        let ctx = admin_ctx();
        let form_app = app.form();

        let form_id = FormId::new(Uuid::new_v4());
        let result = form_app.delete(&ctx, form_id).await;
        assert!(matches!(
            result,
            Err(ApplicationOperationError::OperationFailed(
                DeleteError::NotFound
            ))
        ));
    }
}
