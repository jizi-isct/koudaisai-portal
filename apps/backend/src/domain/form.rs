use crate::application::ports::clock::Clock;
use crate::domain::error::FactoryError;
use crate::domain::form_id::FormId;
use crate::domain::target_specifier::TargetSpecifier;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormType {
    TypeExternal { form_url: String },
}

pub struct Form {
    id: FormId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Uuid,
    updated_by: Uuid,
    targets: Vec<TargetSpecifier>,
    form_name: String,
    summary: String,
    due_date: DateTime<Utc>,
    r#type: FormType,
}


impl Form {
    pub fn register<C: Clock>(
        created_by: Uuid,
        targets: Vec<TargetSpecifier>,
        form_name: String,
        summary: String,
        due_date: DateTime<Utc>,
        r#type: FormType,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        if form_name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Form name is empty".to_string()));
        }

        if summary.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Summary is empty".to_string()));
        }

        let now = clock.now();

        Ok(Self {
            id: FormId::new(Uuid::new_v4()),
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by,
            targets,
            form_name,
            summary,
            due_date,
            r#type,
        })
    }

    pub fn restore(
        id: FormId,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        created_by: Uuid,
        updated_by: Uuid,
        targets: Vec<TargetSpecifier>,
        form_name: String,
        summary: String,
        due_date: DateTime<Utc>,
        r#type: FormType,
    ) -> Self {
        Self {
            id,
            created_at,
            updated_at,
            created_by,
            updated_by,
            targets,
            form_name,
            summary,
            due_date,
            r#type,
        }
    }
}
