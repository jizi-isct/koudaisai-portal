use crate::application::error::{DeleteError, FindError, InsertError, UpdateError};
use crate::application::ports::repositories::form_repo::FormRepo;
use crate::domain::form::Form;
use crate::domain::form_id::FormId;
use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct MemoryFormRepo {
    forms: Arc<RwLock<HashMap<FormId, Form>>>,
}

impl MemoryFormRepo {
    pub fn new() -> Self {
        Self {
            forms: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl FormRepo for MemoryFormRepo {
    async fn find_by_id(&self, id: FormId) -> Result<Option<Form>, FindError> {
        let forms = self
            .forms
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(forms.get(&id).map(|f| {
            Form::restore(
                f.id(),
                f.created_at(),
                f.updated_at(),
                f.created_by(),
                f.updated_by(),
                f.targets().clone(),
                f.name().to_string(),
                f.summary().to_string(),
                f.due_date(),
                f.r#type().clone(),
            )
        }))
    }

    async fn find_all(&self) -> Result<Vec<Form>, FindError> {
        let forms = self
            .forms
            .read()
            .map_err(|e| FindError::InternalError(anyhow!(e.to_string())))?;
        Ok(forms
            .values()
            .map(|f| {
                Form::restore(
                    f.id(),
                    f.created_at(),
                    f.updated_at(),
                    f.created_by(),
                    f.updated_by(),
                    f.targets().clone(),
                    f.name().to_string(),
                    f.summary().to_string(),
                    f.due_date(),
                    f.r#type().clone(),
                )
            })
            .collect())
    }

    async fn insert(&self, form: Form) -> Result<(), InsertError> {
        let mut forms = self
            .forms
            .write()
            .map_err(|e| InsertError::InternalError(anyhow!(e.to_string())))?;
        if forms.contains_key(&form.id()) {
            return Err(InsertError::Conflict);
        }
        forms.insert(form.id(), form);
        Ok(())
    }

    async fn update(&self, form: Form) -> Result<(), UpdateError> {
        let mut forms = self
            .forms
            .write()
            .map_err(|e| UpdateError::InternalError(anyhow!(e.to_string())))?;
        if !forms.contains_key(&form.id()) {
            return Err(UpdateError::NotFound);
        }
        forms.insert(form.id(), form);
        Ok(())
    }

    async fn delete(&self, id: FormId) -> Result<(), DeleteError> {
        let mut forms = self
            .forms
            .write()
            .map_err(|e| DeleteError::InternalError(anyhow!(e.to_string())))?;
        forms.remove(&id).ok_or(DeleteError::NotFound)?;
        Ok(())
    }
}
